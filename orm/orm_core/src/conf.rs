pub struct DbConfig { //classe que será usada para definir sobre a conexão com o BD
    pub host: &'static str,
    pub port: u16,
    pub keyspace: &'static str,
}

impl DbConfig { //classe implementada
    pub fn default() -> Self {
        Self {
            host: "127.0.0.1",
            port: 9042,
            keyspace: "orm_lp",
        }
    }

    pub fn to_url(&self) -> String { //exibir informações, porta e host, ao programador
        format!("{}:{}", self.host, self.port)
    }
}
