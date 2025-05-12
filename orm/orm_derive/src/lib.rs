// // orm_derive/src/lib.rs

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput, Data, Fields};

// #[proc_macro_derive(Model)]
// pub fn derive_model(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = input.ident.clone();
//     let table_name = struct_name.to_string().to_lowercase();

//     let fields = if let Data::Struct(data_struct) = &input.data {
//         if let Fields::Named(fields_named) = &data_struct.fields {
//             &fields_named.named
//         } else {
//             panic!("Esperado struct com campos nomeados");
//         }
//     } else {
//         panic!("Esperado struct");
//     };

//     let field_names: Vec<String> = fields
//         .iter()
//         .map(|f| f.ident.as_ref().unwrap().to_string())
//         .collect();

//     let column_definitions: Vec<String> = fields
//         .iter()
//         .map(|f| {
//             let name = f.ident.as_ref().unwrap().to_string();
//             let ty = quote!(#f).to_string();
//             let scylla_type = if ty.contains("Text") {
//                 "text"
//             } else if ty.contains("Integer") {
//                 "int"
//             } else if ty.contains("Boolean") {
//                 "boolean"
//             } else if ty.contains("Decimal") {
//                 "double"
//             } else if ty.contains("Date") {
//                 "date"
//             } else {
//                 panic!("Tipo não suportado: {}", ty);
//             };
//             format!("{} {}", name, scylla_type)
//         })
//         .collect();

//     let column_definitions_string = column_definitions.join(", ");
//     let field_names_str = field_names.join(", ");
//     let placeholders = (0..field_names.len()).map(|_| "?").collect::<Vec<_>>().join(", ");

//     let insert_query = format!(
//         "INSERT INTO {} ({}) VALUES ({})",
//         table_name,
//         field_names_str,
//         placeholders
//     );

//     let generated = quote! {
//         #[async_trait::async_trait]
//         impl orm_core::Model for #struct_name {
//             fn name_table() -> &'static str {
//                 #table_name
//             }

//             async fn create(&self) {
//                 let query = format!("CREATE TABLE IF NOT EXISTS {} ({}, PRIMARY KEY (id))", #table_name, #column_definitions_string);
//                 orm_core::execute(&query).await;
//             }

//             async fn insert(&self, #( #field_names : String ),*) {
//                 let query = #insert_query;
//                 let values = vec![ #( #field_names.to_string() ),* ];
//                 orm_core::execute_with_params(&query, values).await;
//             }

//             async fn find(&self, id: String) {
//                 let query = format!("SELECT * FROM {} WHERE id = ?", #table_name);
//                 orm_core::execute_with_params(&query, vec![id]).await;
//             }

//             async fn delete(&self, id: String) {
//                 let query = format!("DELETE FROM {} WHERE id = ?", #table_name);
//                 orm_core::execute_with_params(&query, vec![id]).await;
//             }

//             async fn update(&self, id: String, column: String, value: String) {
//                 let query = format!("UPDATE {} SET {} = ? WHERE id = ?", #table_name, column);
//                 orm_core::execute_with_params(&query, vec![value, id]).await;
//             }
//         }
//     };

//     generated.into()
// }


// orm_derive/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();
    let table_name = struct_name.to_string().to_lowercase();

    let fields = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            panic!("Esperado struct com campos nomeados");
        }
    } else {
        panic!("Esperado struct");
    };

    let field_names: Vec<String> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    let column_definitions: Vec<String> = fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap().to_string();
            let ty = quote!(#f).to_string();
            let scylla_type = if ty.contains("Text") {
                "text"
            } else if ty.contains("Integer") {
                "int"
            } else if ty.contains("Boolean") {
                "boolean"
            } else if ty.contains("Decimal") {
                "double"
            } else if ty.contains("Date") {
                "date"
            } else {
                panic!("Tipo não suportado: {}", ty);
            };
            format!("{} {}", name, scylla_type)
        })
        .collect();

    let column_definitions_string = column_definitions.join(", ");
    let field_names_str = field_names.join(", ");
    let placeholders = (0..field_names.len()).map(|_| "?").collect::<Vec<_>>().join(", ");

    let insert_query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        field_names_str,
        placeholders
    );

    let generated = quote! {
        #[async_trait::async_trait]
        impl orm_core::Model for #struct_name {
            fn name_table() -> &'static str {
                #table_name
            }

            async fn create(&self) {
                let query = format!("CREATE TABLE IF NOT EXISTS {} ({}, PRIMARY KEY (id))", #table_name, #column_definitions_string);
                orm_core::execute(&query).await;
            }

            async fn insert(&self, #( #field_names : String ),*) {
                let query = #insert_query;
                let values = vec![ #( #field_names.to_string() ),* ];
                orm_core::execute_with_params(&query, values).await;
            }

            async fn find(&self, id: String) {
                let query = format!("SELECT * FROM {} WHERE id = ?", #table_name);
                orm_core::execute_with_params(&query, vec![id]).await;
            }

            async fn delete(&self, id: String) {
                let query = format!("DELETE FROM {} WHERE id = ?", #table_name);
                orm_core::execute_with_params(&query, vec![id]).await;
            }

            async fn update(&self, id: String, column: String, value: String) {
                let query = format!("UPDATE {} SET {} = ? WHERE id = ?", #table_name, column);
                orm_core::execute_with_params(&query, vec![value, id]).await;
            }
        }
    };

    generated.into()
}
