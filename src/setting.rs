use dotenv::from_filename;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub database_port: u16,
    pub database_name: String,
    pub database_user: String,
    pub database_password: String,

    pub server_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        from_filename("config.env").ok();

        Ok(Settings {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_port: std::env
                ::var("DATABASE_PORT")
                .expect("DATABASE_PORT must be set")
                .parse()
                .expect("DATABASE_PORT must be a valid number"),
            database_name: std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
            database_user: std::env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
            database_password: std::env
                ::var("DATABASE_PASSWORD")
                .expect("DATABASE_PASSWORD must be set"),

            server_port: std::env
                ::var("SERVER_PORT")
                .expect("SERVER_PORT must be set")
                .parse()
                .expect("SERVER_PORT must be a valid number"),
        })
    }

    pub fn get_database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.database_user,
            self.database_password,
            self.database_url,
            self.database_port,
            self.database_name
        )
    }
}
