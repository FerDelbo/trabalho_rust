use anyhow::Result;
use async_trait::async_trait;
use scylla::{Session, QueryResult, SessionBuilder, FromRow};
use scylla::serialize::row::SerializeRow;

static CREATE_KEYSPACE: &str = r#"
CREATE KEYSPACE IF NOT EXISTS teste
    WITH replication = {
        'class': 'NetworkTopologyStrategy',
        'replication_factor': 1
    }
    AND durable_writes = true
"#;

#[async_trait]
pub trait Model: SerializeRow + Send + Sync {
    fn table_name() -> &'static str;
    fn data_fields() -> Vec<(&'static str, &'static str)>;

    async fn connect(url: &str) -> Result<Session> {
        let session = SessionBuilder::new()
            .known_node(url)
            .build()
            .await?;
        session.query_unpaged(CREATE_KEYSPACE, &[]).await?;
        Ok(session)
    }

    async fn create_table(session: &Session) -> Result<()> {
        let fields = Self::data_fields();

        let mut cql_fields: Vec<String> = fields
            .iter()
            .map(|(name, rust_type)| {
                let cql_type = match *rust_type {
                    "String" => "text",
                    "i32" => "int",
                    "i64" => "bigint",
                    "bool" => "boolean",
                    "f32" => "float",
                    "f64" => "double",
                    _ => "text",
                };
                format!("{} {}", name, cql_type)
            })
            .collect();

        if let Some((pk, _)) = fields.first() {
            cql_fields.push(format!("PRIMARY KEY ({})", pk));
        }

        let query = format!(
            "CREATE TABLE IF NOT EXISTS teste.{} ({})",
            Self::table_name(),
            cql_fields.join(", ")
        );

        session.query_unpaged(query, &[]).await?;
        Ok(())
    }

    async fn insert_row(&self, session: &Session) -> Result<QueryResult> {
        let fields = Self::data_fields();
        let field_names: Vec<&str> = fields.iter().map(|(name, _)| *name).collect();
        let placeholders = vec!["?"; field_names.len()].join(", ");

        let query = format!(
            "INSERT INTO teste.{} ({}) VALUES ({})",
            Self::table_name(),
            field_names.join(", "),
            placeholders
        );

        let result = session.query_unpaged(query, self).await?;
        Ok(result)
    }

    // async fn find_by_id(&self, session: &Session, id: i32) -> Result<QueryResult> {
    //     let template_query = "SELECT * FROM teste.pessoa; WHERE {}";
    // }
}
