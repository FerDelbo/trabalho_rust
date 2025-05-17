use anyhow::Ok;
use scylla::{deserialize::{result, row}, transport::session, FromRow, SerializeRow, SerializeValue, Session, SessionBuilder, QueryResult};

static CREATE_KEYSPACE: &str = r#"
CREATE KEYSPACE IF NOT EXISTS teste
    WITH replication = {
        'class': 'NetworkTopologyStrategy', 
        'replication_factor' : 1
    }
    AND durable_writes = true
"#;

pub trait Model {

    async fn create(&self, name_table: &str, session: Session) -> Result<QueryResult, anyhow::Error> {
        let fields = Self::data_fields();
    
        // Mapeia os campos para tipos CQL
        let mut fields_cql: Vec<String> = Vec::new();
        for (i, (nome, tipo)) in fields.iter().enumerate() {
            let cql_type = match *tipo {
                "String" => "text",
                "i32" => "int",
                "i64" => "bigint",
                "bool" => "boolean",
                "f32" => "float",
                "f64" => "double",
                _ => "text", // fallback genérico
            };
    
            fields_cql.push(format!("{} {}", nome, cql_type));
        }
    
        // Adiciona a PRIMARY KEY (usamos o primeiro campo como chave primária)
        if let Some((pk, _)) = fields.first() {
            let fields_str = format!("{}, PRIMARY KEY ({})", fields_cql.join(", "), pk);
            let query = format!("CREATE TABLE IF NOT EXISTS teste.{} ({})", name_table, fields_str);
            println!("Query: {}", query);
            let result = session.query_unpaged(query, ()).await?;
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Nenhum campo definido em data_fields()"))
        }
    }

    async fn connect(&self, url: &str) -> Session {
        let session = SessionBuilder::new()
            .known_node(url)
            .build()
            .await
            .expect("Connection Refused!");

        session.query_unpaged(CREATE_KEYSPACE, ()).await;
        session
    }

    fn data_fields() ->  Vec<(&'static str, &'static str)>;

}
