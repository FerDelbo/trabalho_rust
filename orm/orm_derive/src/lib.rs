use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Model)]
pub fn derive_modelo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let nome_struct = input.ident.clone();

    let nome_tabela = nome_struct.to_string().to_lowercase();

    let generated_code = quote! {
        impl orm_core::Model for #nome_struct {
            fn name_table() -> &'static str {
                #nome_tabela
            }
            fn find(&self){
                println!("Buscando...")
            }
            fn insert(&self) {
                println!("INSERT INTO {} ...", Self::name_table());
            }
            
            fn create(&self) {
                println!("CREATE TABLE {} ...", Self::name_table())
            }

            fn delete(&self) {
                println!("Deletenado...")
            }

            fn update(&self){
                println!("UPDATE INTO TO {} ...", Self::name_table())
            }

        }
    };

    generated_code.into()
}
