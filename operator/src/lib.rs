pub mod utils {
    use std::fmt;

    pub enum AppError {
        UserNotFound,
        NoData,
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::UserNotFound => write!(f, "User not found"),
                Self::NoData => write!(f, "No data available for this user"),
            }
        }
    }
    pub fn get_user_id(username: &str) -> Result<u32, AppError> {
        match username {
            "yannis" => Ok(42),
            _ => Err(AppError::UserNotFound),
        }
    }
    pub fn load_data(user_id: u32) -> Result<String, AppError> {
        match user_id {
            42 => Ok("top-secret-data".to_string()),
            _ => Err(AppError::NoData),
        }
    }
    pub fn process_user_data(username: &str) -> Result<String, AppError> {
        let user_id = get_user_id(username)?;
        let data = load_data(user_id)?;
        Ok(format!("Data for {username}: {data}"))
    }
}
