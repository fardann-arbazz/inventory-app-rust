use std::io::{self, Write};

use crate::{errors::users_error::UsersError, models::users::Role};

// function untuk input index users
pub fn get_index_users() -> Result<usize, UsersError> {
    print!("Masukkan nomor users: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| UsersError::InvalidInput("Input tidak valid".to_string()))?;

    let index_input = input
        .trim()
        .parse::<usize>()
        .map_err(|_| UsersError::InvalidInput("Masukan nomor users yang valid".to_string()))?;

    Ok(index_input)
}

// function untuk input username
pub fn get_username() -> Result<String, UsersError> {
    print!("Masukan username: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| UsersError::InvalidInput("Gagal membaca input username".to_string()))?;

    let username = input.trim().to_string();

    if username.is_empty() {
        Err(UsersError::InvalidInput(
            "Username tidak boleh kosong".to_string(),
        ))
    } else {
        Ok(username)
    }
}

// function untuk input password
pub fn get_password() -> Result<String, UsersError> {
    print!("Masukan password: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| UsersError::InvalidInput("Gagal membaca input password".to_string()))?;

    let password = input.trim().to_string();

    if password.is_empty() {
        Err(UsersError::InvalidInput(
            "Password tidak boleh kosong".to_string(),
        ))
    } else {
        Ok(password)
    }
}

pub fn get_role() -> Result<Role, UsersError> {
    print!("Masukan role (admin/kasir): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| UsersError::InvalidInput("Gagal membaca input role".to_string()))?;

    let role = match input.trim() {
        "admin" => Role::Admin,
        "kasir" => Role::Kasir,
        _ => return Err(UsersError::InvalidInput("Role tidak valid".to_string())),
    };

    Ok(role)
}
