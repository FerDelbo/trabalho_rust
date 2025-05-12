use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::sync::Arc;
use tokio::sync::OnceCell;
// use scylla::query::QueryExecutor;

static SESSION: OnceCell<Arc<Session>> = OnceCell::const_new();

pub async fn get_session() -> Arc<Session> {
    SESSION
        .get_or_init(|| async {
            let session = SessionBuilder::new()
                .known_node("127.0.0.1:9042")
                .build()
                .await
                .expect("Não foi possível conectar ao ScyllaDB");
            Arc::new(session)
        })
        .await
        .clone()
}

pub async fn execute(query: &str) {
    let session = get_session().await;
    session.query_unpaged(query, &[]).await.expect("Erro ao executar query");
}

