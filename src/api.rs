pub enum ApiKey {
    Keys { main: String, secret: String },
}

impl ApiKey {
    pub fn new() -> Self {
        Self::Keys {
            main: dotenv::var("API_MAIN").expect("API_MAIN not found"),
            secret: dotenv::var("API_SECRET").expect("API_SECRET not found"),
        }
    }

    pub fn get_main(&self) -> Option<String> {
        match self {
            Self::Keys { main, .. } => Some(main.to_owned()),
        }
    }

    pub fn get_secret(&self) -> Option<String> {
        match self {
            Self::Keys { secret, .. } => Some(secret.to_owned()),
        }
    }
}
