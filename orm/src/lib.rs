use anyhow::Result;
use async_trait::async_trait;
use scylla::{Session, QueryResult, SessionBuilder, FromRow};
use scylla::serialize::row::SerializeRow;
use scylla::frame::response::result::Row;

static CREATE_KEYSPACE: &str = r#"
CREATE KEYSPACE IF NOT EXISTS teste
    WITH replication = {
        'class': 'NetworkTopologyStrategy',
        'replication_factor': 1
    }
    AND durable_writes = true
"#;


#[async_trait]
pub trait Model: SerializeRow + Send + Sync + FromRow{
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

    async fn find_by_id(session: &Session, id: i32) -> Result<Vec<Row>> {
        let pk = Self::data_fields()
            .first()
            .expect("Nenhum campo em data_fields")
            .0;
    
        let query = format!("SELECT * FROM teste.{} WHERE {} = ?", Self::table_name(), pk);
    
        // Executa a query com o ID como parâmetro
        let result = session.query_unpaged(query, (id,)).await?;
    
        // Coleta as linhas do resultado e retorna
        let rows = result.rows.unwrap_or_default();
        Ok(rows)
    }


    async fn update_row(&self, session: &Session, primary_key_fields: &[&str]) -> Result<QueryResult> {
        let fields = Self::data_fields();
        
        // Filter out primary key fields from the update set
        let update_fields: Vec<&str> = fields.iter()
            .map(|(name, _)| *name)
            .filter(|name| !primary_key_fields.contains(name))
            .collect();
        
        // Create SET clause with placeholders
        let set_clause = update_fields.iter()
            .map(|field| format!("{} = ?", field))
            .collect::<Vec<_>>()
            .join(", ");
        
        // Create WHERE clause for primary key
        let where_clause = primary_key_fields.iter()
            .map(|field| format!("{} = ?", field))
            .collect::<Vec<_>>()
            .join(" AND ");
        
        let query = format!(
            "UPDATE teste.{} SET {} WHERE {}",
            Self::table_name(),
            set_clause,
            where_clause
        );
        
        let result = session.query_unpaged(query, self).await?;
        Ok(result)
    }
    
    async fn delete_row(session: &Session, id: i32) -> Result<QueryResult> {
        let pk = Self::data_fields()
            .first()
            .expect("No fields in data_fields")
            .0;

        let query = format!(
            "DELETE FROM teste.{} WHERE {} = ?", 
            Self::table_name(), 
            pk
        );

        let result = session.query_unpaged(query, (id,)).await?;
        Ok(result)
    }

}
