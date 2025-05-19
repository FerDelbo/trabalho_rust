// use anyhow::{Error, Ok};
// use scylla::{deserialize::{result, row}, FromRow, SerializeRow, SerializeValue, SessionBuilder};
// use orm::{Model};    

// static CREATE_KEYSPACE: &str = r#"
// CREATE KEYSPACE IF NOT EXISTS teste
//     WITH replication = {
//         'class': 'NetworkTopologyStrategy', 
//         'replication_factor' : 1
//     }
//     AND durable_writes = true
// "#;

// static CREATE_MESSAGES_TABLE: &str = r#"
// CREATE TABLE IF NOT EXISTS messaging.messages (
//     channel_id int,
//     message_id int,
//     author text,
//     content text,
//     PRIMARY KEY (channel_id, message_id)
// );
// "#;

// static SELECT_MESSAGES_QUERY: &str = "SELECT channel_id, message_id, author, content FROM messages WHERE channel_id = ?";

// static CURRENT_KEYSPACE: &str = "teste";

// static  INSERT_MESSAGES_QUERY: &str = "INSERT INTO messages (channel_id, message_id, author, content) VALUES (?, ?, ?, ?)";

// #[derive(SerializeRow, FromRow, Clone)]
// struct Message {
//     channel_id: i32 ,
//     message_id: i32,
//     author: String,
//     content: String
// }

// struct Pessoa{
//     id: i32,
//     nome: String,
//     idade: i32,
// }

// impl Model for Pessoa {
//     fn table_name(&self) -> &str {
//         "Pessoa"
//     }
    
//     fn data_fields() ->  Vec<(&'static str, &'static str)> {
//         vec![("nome", "String"), ("idade", "i32")]
//     }
// } 

// #[tokio::main]
// async fn main() {
// // fn main(){
//     // println!("Hello, world!");

//     let pessoa = Pessoa{
//         id: 1,
//         nome: "Fernando".to_string(),
//         idade: 21,
//     };

//     let session = pessoa.connect("172.17.0.2:9042").await;
//     session.use_keyspace(CURRENT_KEYSPACE, true).await;

//     // antes de criar o programador deve reescrever o metodo data_fields para indicar os campos de criação da tabela
//     let query = pessoa.create(session).await;
// }
//     // println!("{}", query)


//     // //preparação de ambiente
//     // let session = SessionBuilder::new()
//     //     .known_node("172.17.0.2:9042")
//     //     .build()
//     //     .await
//     //     .expect("Connection Refused!");

//     // session.query_unpaged(CREATE_KEYSPACE, ()).await?;
//     // session.query_unpaged(CREATE_MESSAGES_TABLE, ()).await?;

//     // session.use_keyspace(CURRENT_KEYSPACE, true).await?;

//     // //insrindo novos dados
    
//     // let message =  Message {
//     //     channel_id: 1 ,
//     //     message_id: 2,
//     //     author: "gabriel".to_string(),
//     //     content: "Salve".to_string()
//     // };

//     // let prepare_insert = session.prepare(INSERT_MESSAGES_QUERY).await?;
//     // session.execute_unpaged(&prepare_insert, &message).await?;
//     // // session.query_unpaged(INSERT_MESSAGES_QUERY, message).await?;

//     // //mostrando mensagens
//     // // let select_query: &str = "SELECT channel_id, message_id, author, content FROM messages";
//     // let prepare_select = session.prepare(SELECT_MESSAGES_QUERY).await?;
    
//     // // let result = session.query_unpaged(SELECT_MESSAGES_QUERY, ()).await?;
//     // let result = session.execute_unpaged(&prepare_select, (&message.channel_id,)).await?;

//     // let mut rows = result.rows_typed::<Message>()?;


//     // // let result = session.query_unpaged("SELECT address, username FROM system.clients", ()).await?;// onde stá vazio requer um where, e o que
    
//     // // let mut rows =  result.rows_typed::<(IpAddr, String)>()?;

//     // while let Some(row) = rows.next().transpose()?{
//     //     // println!("IP: {} for {} ", row.0, row.1);
//     //     println!("{}: {}", row.author, row.content)
//     // }
    

//     // let prepare_delete = session.prepare("DELETE FROM messages WHERE channel_id = ? AND message_id = ?").await?;
//     // session.execute_unpaged(&prepare_delete, (1,2)).await?;
//     // Ok(())
// // }

use scylla::{FromRow, Session};
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session = Pessoa::connect("172.17.0.2:9042").await?;
    session.use_keyspace("teste", true).await?;

    // Pessoa::create_table(&session).await?;

    // Instanciou => já insere
    let mut nova = Pessoa {
        id: 1,
        nome: "Fernando".to_string(),
        idade: 21,
    };

    nova.insert_row(&session).await?;
   
    let _p = Pessoa::new(2, "Gabriel".to_string(), 22, &session).await?;

    let rows = Pessoa::find_by_id(&session, 1).await?;
    
    for row in rows {
        for (i, col) in row.columns.iter().enumerate() {
            println!("{}: {:?}", i, col);
        }
    }
    
    nova.nome = "Joca".to_string();
    nova.idade = 33;
    nova.update_row(&session, &["id"]).await?;

    println!("{:?}", nova);
    Ok(())
}
