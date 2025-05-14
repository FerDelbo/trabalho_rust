use scylla::{
    FromRow, Session, SessionBuilder, IntoTypedRows,
};
use scylla::serialize::row::SerializeRow;
use async_trait::async_trait;
use std::sync::Arc;
use anyhow::Result;

#[async_trait]
pub trait ModelBase: Sized + SerializeRow + FromRow + Send + Sync {
    fn keyspace() -> &'static str;
    fn create_keyspace_cql() -> &'static str;
    fn create_table_cql() -> &'static str;
    fn insert_cql() -> &'static str;
    fn select_cql() -> &'static str;
    fn delete_cql() -> &'static str;

    async fn connect(uri: &str) -> Result<Arc<Session>> {
        let session = SessionBuilder::new()
            .known_node(uri)
            .build()
            .await?;
        session.query_unpaged(Self::create_keyspace_cql(), ()).await?;
        session.use_keyspace(Self::keyspace(), true).await?;
        Ok(Arc::new(session))
    }

    async fn create_table(session: &Session) -> Result<()> {
        session.query_unpaged(Self::create_table_cql(), ()).await?;
        Ok(())
    }

    async fn insert(&self, session: &Session) -> Result<()> {
        let stmt = session.prepare(Self::insert_cql()).await?;
        session.execute_unpaged(&stmt, self).await?;
        Ok(())
    }

    async fn find(session: &Session, keys: impl SerializeRow + Send) -> Result<Vec<Self>> {
        let stmt = session.prepare(Self::select_cql()).await?;
        let result = session.execute_unpaged(&stmt, keys).await?;

        let rows = result.rows.ok_or_else(|| anyhow::anyhow!("No rows found"))?;
        let typed_rows = rows.into_typed::<Self>();

        let mut output = Vec::new();
        for row in typed_rows {
            output.push(row?)
        }
        Ok(output)
    }

    async fn delete(session: &Session, keys: impl SerializeRow + Send) -> Result<()> {
        let stmt = session.prepare(Self::delete_cql()).await?;
        session.execute_unpaged(&stmt, keys).await?;
        Ok(())
    }
}
