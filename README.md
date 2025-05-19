# trabalho_rust
Criação de um Object Relational Mapping (ORM) com a linguagem de programação Rust.

# Ferramentas necessárias para execução do código
     - Docker:
        - Necessário para rodar a imagem do banco de dados do ScyllaDB;
     - rustup:
        - Necessário para instalar e gerenciar diferentes versões do compilador Rust;

# Requisitos para a execução do programa
    - Ligar a imagem do bando de dados para execução;
    - utilizar o IP da máquina atual;

# Como compilar o código
    - cargo build;
    - cargo run;

# Comparativo entre a abordagem adotada e soluções existentes na linguagem
    - ScyllaDB (NoSQL): Um banco de dados distribuído compatível com Apache Cassandra.
    - Diesel (SQL): Um ORM tradicional para bancos SQL (PostgreSQL, MySQL, SQLite).
    - SeaORM (SQL Async): Um ORM moderno e assíncrono para bancos SQL.

    No projeto foi utilizado uma abordagem personalizada (ScyllaDB), um pouco diferente dos métodos comuns utlizados
    como Diesel e SeaORM. Pelo fato do ScyllaDB ser uma linguagem NoSQL outras tipos de abordagem não são viavéis por serem SQL.
    Para construir uma conexão com o banco de dados construimos uma conexão manual com ele, e usamos comandos diretos como session.query() e .bind() manualmente para a execução de queries. Em contra partida o métodos Diesel e SeaORM utiliza url de conexão e é baseado em async com Database::connec (...) respectivamente, e ambos não são suportados pelo ScyllaDB, os outros métodos utilizam macros (insert_into, filter) e comandos em DSL (User::insert().exec), que são abordagens mais alto nível, seguro e genérico.
    
    ![Tabela de comparações](/Capturar.PNG)

    ScyllaDB
        - Driver para bancos compatíveis com Apache Cassandra.
        - Otimizado para alta escalabilidade.
        - Queries via CQL (Cassandra Query Language).
        - Sistemas distribuídos (ex.: IoT, análises em tempo real).
    
    Diesel
        - ORM síncrono com verificação estática de queries em tempo de compilação.
        - Performance próxima do SQL puro.
        - Migrações automatizadas via CLI.
        - Schema rígido (alterações exigem migrações).
        - Aplicações com modelos de dados estáveis (ex.: sistemas financeiros).

    SeaORM
        - ORM assíncrono baseado no padrão Active Record.
        - Suporte nativo a async/await.
        - Relacionamentos declarativos (has_many, belongs_to).
        - Overhead maior que Diesel.
        - APIs modernas (ex.: backends para aplicações web).

    Considerações:
        ORMs (Diesel/SeaORM) são ideais para produtividade em aplicações com schemas bem definidos. 
        NoSQL (Scylla) é indispensável para cenários de alta escalabilidade ou dados flexíveis.
        A escolha da ferramenta deve considerar:    
            - Natureza do projeto (SQL vs. NoSQL).   
            - Requisitos de performance.
            - Necessidade de assincronismo.
            
# Diferentes tipos de conexão com o banco de dados

```rust
    ScyllaDB
    use scylla::{Session, SessionBuilder};
    async fn connect() -> Result<Session, scylla::transport::errors::NewSessionError> {
        SessionBuilder::new()
            .known_node("127.0.0.1:9042")
            .build()
            .await
    }

    Diesel
    diesel::pg::PgConnection::establish("postgres://user:pass@localhost/db")?;

    SeaORM
    Database::connect("postgres://user:pass@localhost/db").await?;
    } 
