#[derive(Debug)]
pub enum UsersError {
    NotFound,
    InvalidInput(String),
    InvalidCredentials,
    UserAlreadyExists,
}

impl std::fmt::Display for UsersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UsersError::UserAlreadyExists => write!(f, "User sudah ada"),
            UsersError::NotFound => write!(f, "User tidak ditemukan"),
            UsersError::InvalidInput(msg) => write!(f, "Input tidak valid: {}", msg),
            UsersError::InvalidCredentials => write!(f, "Username atau Password tidak valid"),
        }
    }
}
