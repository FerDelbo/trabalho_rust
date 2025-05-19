use anyhow::Result;
use async_trait::async_trait;
use scylla::{Session, QueryResult, SessionBuilder, FromRow};
use scylla::serialize::row::SerializeRow;
use scylla::frame::response::result::Row;

// Query de criar KEYSPACE
static CREATE_KEYSPACE: &str = r#"
CREATE KEYSPACE IF NOT EXISTS teste
    WITH replication = {
        'class': 'NetworkTopologyStrategy',
        'replication_factor': 1
    }
    AND durable_writes = true
"#;

// Habilitando trait de assíncrono
#[async_trait]
pub trait Model: SerializeRow + Send + Sync + FromRow{
    fn table_name() -> &'static str;
    fn data_fields() -> Vec<(&'static str, &'static str)>;
    //Função de conectar
    async fn connect(url: &str) -> Result<Session> {
        //Cria Sessão
        let session = SessionBuilder::new()
            .known_node(url)
            .build()
            .await?;

        //Query de criar Keyspace
        session.query_unpaged(CREATE_KEYSPACE, &[]).await?;
        Ok(session)
    }
    //Função criar tabela
    async fn create_table(session: &Session) -> Result<()> {
        let fields = Self::data_fields();
        //Padroniza campos a serem aceitos
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
        //Definir primary key como o primeiro elemento passado da struct
        if let Some((pk, _)) = fields.first() {
            cql_fields.push(format!("PRIMARY KEY ({})", pk));
        }
        let query = format!(
            "CREATE TABLE IF NOT EXISTS teste.{} ({})",
            Self::table_name(),
            cql_fields.join(", ")
        );
        
        // Executa query de create table, como o nome e seus campos
        session.query_unpaged(query, &[]).await?;
        Ok(())
    }
    //Função de inserir dado em tabela
    async fn insert_row(&self, session: &Session) -> Result<QueryResult> {
        let fields = Self::data_fields();
        let field_names: Vec<&str> = fields.iter().map(|(name, _)| *name).collect();
        //Placeholders para saber a quantidade necessária de informações de entrada
        let placeholders = vec!["?"; field_names.len()].join(", ");
        //Query de inserção
        let query = format!(
            "INSERT INTO teste.{} ({}) VALUES ({})", 
            Self::table_name(),
            field_names.join(", "),
            placeholders
        );
        //Executa query de inserção
        let result = session.query_unpaged(query, self).await?;
        Ok(result)
    }
    //Função de retrieve por Id
    async fn find_by_id(session: &Session, id: i32) -> Result<Vec<Row>> {
        //Seleciona primary key como primeiro campo passado
        let pk = Self::data_fields()
            .first()
            .expect("Nenhum campo em data_fields")
            .0;
    
        let query = format!("SELECT * FROM teste.{} WHERE {} = ?", Self::table_name(), pk);
    
        // Executa a query de seleção com o Id como parâmetro
        let result = session.query_unpaged(query, (id,)).await?;
    
        // Coleta as linhas do resultado e retorna
        let rows = result.rows.unwrap_or_default();
        Ok(rows)
    }

    //Função de atualizar linha em tabela
    async fn update_row(&self, session: &Session, primary_key_fields: &[&str]) -> Result<QueryResult> {
        let fields = Self::data_fields();
        
        //Filtra primary keys do Update Set
        let update_fields: Vec<&str> = fields.iter()
            .map(|(name, _)| *name)
            .filter(|name| !primary_key_fields.contains(name))
            .collect();
        
        //Cria clausa SET com os placeholders
        let set_clause = update_fields.iter()
            .map(|field| format!("{} = ?", field))
            .collect::<Vec<_>>()
            .join(", ");
        
        //Cria clausa WHERE para primary key
        let where_clause = primary_key_fields.iter()
            .map(|field| format!("{} = ?", field))
            .collect::<Vec<_>>()
            .join(" AND ");
        //Cria query para executar clausulas e atualizar com base na primary key        
        let query = format!(
            "UPDATE teste.{} SET {} WHERE {}",
            Self::table_name(),
            set_clause,
            where_clause
        );
        //Executa query de atualização
        let result = session.query_unpaged(query, self).await?;
        Ok(result)
    }
    //Cria função de deletar linha
    async fn delete_row(session: &Session, id: i32) -> Result<QueryResult> {
        let pk = Self::data_fields()
            .first()
            .expect("No fields in data_fields")
            .0;
        //Criar query de deleção de linha
        let query = format!(
            "DELETE FROM teste.{} WHERE {} = ?", 
            Self::table_name(), 
            pk
        );
        //Executa query de deleção
        let result = session.query_unpaged(query, (id,)).await?;
        Ok(result)
    }

}
