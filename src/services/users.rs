use crate::{
    errors::users_error::UsersError,
    models::users::{Role, User},
};

pub struct UserService {
    pub users: Vec<User>,
    pub current_user: Option<User>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: vec![
                User {
                    id: 1,
                    username: "admin".to_string(),
                    password: "admin123".to_string(),
                    role: Role::Admin,
                },
                User {
                    id: 2,
                    username: "kasir1".to_string(),
                    password: "kasir123".to_string(),
                    role: Role::Kasir,
                },
            ],
            current_user: None,
        }
    }

    // function untuk melihat users
    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    // function untuk login
    pub fn login(&mut self, username: &str, password: &str) -> Result<bool, UsersError> {
        if let Some(user) = self
            .users
            .iter()
            .find(|user| user.username == username && user.password == password)
        {
            self.current_user = Some(user.clone());
            Ok(true)
        } else {
            Err(UsersError::InvalidCredentials)
        }
    }

    // function untuk logout
    pub fn logout(&mut self) {
        self.current_user = None;
    }

    // function untuk menambahkan users
    pub fn add_user(
        &mut self,
        username: String,
        password: String,
        role: Role,
    ) -> Result<(), UsersError> {
        if self.users.iter().any(|u| u.username == username) {
            Err(UsersError::UserAlreadyExists)
        } else {
            self.users.push(User {
                id: self.users.len() as u32 + 1,
                username,
                password,
                role,
            });
            Ok(())
        }
    }

    // function untuk mengupdate users
    pub fn update_users(
        &mut self,
        index: usize,
        new_username: String,
        new_password: String,
        new_role: Role,
    ) -> Result<(), UsersError> {
        if index == 0 || index > self.users.len() {
            Err(UsersError::NotFound)
        } else {
            let user = &mut self.users[index - 1];
            user.username = new_username;
            user.password = new_password;
            user.role = new_role;
            Ok(())
        }
    }

    // function untuk menghapus users
    pub fn delete_users(&mut self, index: usize) -> Result<(), UsersError> {
        if index == 0 || index > self.users.len() {
            Err(UsersError::NotFound)
        } else {
            self.users.remove(index - 1);
            Ok(())
        }
    }

    // function untuk mendapatkan user yang sedang login
    pub fn get_current_user(&self) -> Option<User> {
        self.current_user.clone()
    }
}
