use std::clone;

use scylla::{FromRow, SerializeRow, Session};
use orm::Model;

#[derive(Debug, Clone, FromRow, scylla::SerializeRow)]
struct Pessoa {
    id: i32,
    nome: String,
    idade: i32,
}

impl Model for Pessoa {
    fn table_name() -> &'static str {
        "pessoa"
    }

    fn data_fields() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "i32"),
            ("nome", "String"),
            ("idade", "i32"),
        ]
    }
}

impl Pessoa {
    pub async fn new(id: i32, nome: String, idade: i32, session: &Session) -> anyhow::Result<Self> {
        let pessoa = Pessoa { id, nome, idade };
        pessoa.insert_row(session).await?;
        Ok(pessoa)
    }
}


#[derive(Clone, FromRow, Debug, SerializeRow)]
struct Cidade {
    nome: String,
    estado: String
}

impl Model for Cidade {
    fn table_name() -> &'static str {
        "Cidade"
    }

    fn data_fields() -> Vec<(&'static str, &'static str)> {
        vec![
            ("nome", "String"),
            ("estado", "String"),
        ]
    }
}

impl Cidade {
    pub async fn new(nome: String,  estado: String, session: &Session) -> anyhow::Result<Self> {
        let cidade = Cidade {nome, estado };
        cidade.insert_row(session).await?;
        Ok(cidade)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session = Pessoa::connect("172.17.0.2:9042").await?;
    session.use_keyspace("teste", true).await?;

    // Pessoa::create_table(&session).await?;

    let mut nova = Pessoa {
        id: 1,
        nome: "Fernando".to_string(),
        idade: 21,
    };
    
    nova.insert_row(&session).await?;
    
    // Instanciou => já insere
    let _p = Pessoa::new(2, "Gabriel".to_string(), 22, &session).await?;

    let rows = Pessoa::find_by_id(&session, 1).await?;
    
    for row in rows {
        for (i, col) in row.columns.iter().enumerate() {
            println!("{}: {:?}", i, col);
        }
    }
    
    nova.nome = "Xina".to_string();
    nova.idade = 33;
    nova.update_row(&session, &["id"]).await?;

    println!("{:?}", nova);
    let delete_result = Pessoa::delete_row(&session, 1).await?;
    // println!("Deleted {} rows", delete_result.rows_num);

    Cidade::create_table(&session).await?;
    let cerquilho = Cidade::new("Cerquilho".to_string(), "São Paulo".to_string(), &session).await?;
    println!("{:?}", cerquilho);
    
    Ok(())
}
