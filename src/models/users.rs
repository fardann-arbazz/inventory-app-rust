#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Admin,
    Kasir,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub role: Role,
}
