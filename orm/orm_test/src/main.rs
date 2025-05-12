use orm_core::types::{Integer, Text, Boolean};
use orm_derive::Model;

#[derive(Model)]
pub struct Usuario {
    pub id: Integer,
    pub nome: Text,
    pub email: Text,
    pub ativo: Boolean,
}

#[tokio::main]
async fn main() {
    let user = Usuario {};


    user.create().await;

    user.insert("1".to_string(), "Fernando".to_string(), "email.com".to_string(), "false".to_string()).await;

    user.find("1".to_string()).await;

    user.update("1".to_string(), "nome".to_string(), "Fernandinho".to_string()).await;

    user.delete("1".to_string()).await;
}
