extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields, Meta, NestedMeta, Lit};


#[proc_macro_derive(ModelBase, attributes(primary_key))]
pub fn model_base_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let table_name = name.to_string().to_lowercase();

    // Extrai campos
    let fields = if let Data::Struct(DataStruct { fields: Fields::Named(f), .. }) = &input.data {
        f
    } else {
        panic!("ModelBase só pode ser usado em structs com campos nomeados");
    };

    let mut field_names = Vec::new();
    let mut primary_keys = Vec::new();

    // Extrai os nomes dos campos
    for field in &fields.named {
        let ident = field.ident.as_ref().unwrap().to_string();
        field_names.push(ident);
    }

    // Extrai #[primary_key(...)]
    for attr in input.attrs.iter().filter(|a| a.path.is_ident("primary_key")) {
        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            for nested in meta_list.nested.iter() {
                if let NestedMeta::Lit(Lit::Str(lit_str)) = nested {
                    primary_keys.push(lit_str.value());
                }
            }
        }
    }

    if primary_keys.is_empty() {
        panic!("Você deve especificar #[primary_key(...)] na struct.");
    }

    let fields_cql = field_names.join(", ");
    let placeholders = vec!["?"; field_names.len()].join(", ");
    let select_where = primary_keys.iter().map(|k| format!("{} = ?", k)).collect::<Vec<_>>().join(" AND ");

    let create_fields = field_names.iter().map(|f| format!("{} text", f)).collect::<Vec<_>>().join(", ");
    let primary_clause = format!("PRIMARY KEY ({})", primary_keys.join(", "));

    let create_stmt = format!(
        "CREATE TABLE IF NOT EXISTS {} ({}{}, {})",
        table_name,
        create_fields,
        if create_fields.is_empty() { "" } else { ", " },
        primary_clause
    );

    let insert_stmt = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        fields_cql,
        placeholders
    );

    let select_stmt = format!(
        "SELECT {} FROM {} WHERE {}",
        fields_cql,
        table_name,
        select_where
    );

    let delete_stmt = format!(
        "DELETE FROM {} WHERE {}",
        table_name,
        select_where
    );

    let expanded = quote! {
        #[async_trait::async_trait]
        impl ModelBase for #name {
            fn keyspace() -> &'static str {
                "messaging"
            }
            fn table_name() -> &'static str {
                #table_name
            }
            fn create_table_cql() -> &'static str {
                #create_stmt
            }
            fn insert_cql() -> &'static str {
                #insert_stmt
            }
            fn select_cql() -> &'static str {
                #select_stmt
            }
            fn delete_cql() -> &'static str {
                #delete_stmt
            }
        }
    };

    TokenStream::from(expanded)
}