use crate::{
    errors::users_error::UsersError,
    menu::MenuExitStatus,
    models::users::Role,
    services::users::UserService,
    utils::{audit_logs, input_users},
};

//  function untuk handle login
pub fn handle_login(service: &mut UserService) -> Result<(), UsersError> {
    println!();
    println!("====================");
    println!("== Login ==");
    println!("====================");
    println!();

    let username = input_users::get_username()?;
    let password = input_users::get_password()?;

    let is_logged_in = service.login(&username, &password)?;

    if is_logged_in {
        audit_logs::add_logs(&username, "Login");
        println!("Login berhasil!");
    } else {
        println!("Login gagal!");
    }

    Ok(())
}

// function untuk handle logout
pub fn handle_logout(service: &mut UserService) -> Result<MenuExitStatus, UsersError> {
    println!();
    println!("====================");
    println!("== Logout ==");
    println!("====================");
    println!();

    if let Some(user) = service.get_current_user() {
        audit_logs::add_logs(&user.username, "Logout");
        service.logout();
        println!("Logout berhasil!");
    } else {
        println!("Anda tidak login!");
    }

    Ok(MenuExitStatus::Logout)
}

// function untuk handle update users
pub fn handle_update_users(service: &mut UserService) -> Result<(), UsersError> {
    println!();
    println!("====================");
    println!("== Update Users ==");
    println!("====================");
    println!();

    handle_users_list(service);

    let index_input = input_users::get_index_users()?;
    let username = input_users::get_username()?;
    let password = input_users::get_password()?;
    let role = input_users::get_role()?;

    audit_logs::add_logs(&username, "Mengupdate users"); // Menambahkan log

    service.update_users(index_input, username, password, role)?;
    println!("Users berhasil diupdate!");

    Ok(())
}

// function untuk handle delete users
pub fn handle_delete_users(service: &mut UserService) -> Result<(), UsersError> {
    println!();
    println!("====================");
    println!("== Delete Users ==");
    println!("====================");
    println!();

    handle_users_list(service);

    let index_input = input_users::get_index_users()?;

    audit_logs::add_logs("admin", "Menghapus users"); // Menambahkan log

    service.delete_users(index_input)?;
    println!("Users berhasil dihapus!");

    Ok(())
}

// function untuk handle lihat users
pub fn handle_users_list(service: &UserService) {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");

    // Header with colored title
    println!("\n\x1B[1;34m╔══════════════════════════════════════════════╗");
    println!("║                 DAFTAR USER                  ║");
    println!("╠══════╦═══════════════╦═══════════════╦═══════╣");
    println!(
        "║ \x1B[1m{:4}\x1B[0m ║ \x1B[1m{:13}\x1B[0m ║ \x1B[1m{:13}\x1B[0m ║ \x1B[1m{:5}\x1B[0m ║",
        "No", "Username", "Password", "Role"
    );
    println!("╠══════╬═══════════════╬═══════════════╬═══════╣\x1B[0m");

    let users = service.get_users();

    // Table rows
    for (i, user) in users.iter().enumerate() {
        let role = match user.role {
            Role::Admin => "\x1B[33mAdmin\x1B[0m", // Yellow for admin
            Role::Kasir => "\x1B[32mKasir\x1B[0m", // Green for kasir
        };

        println!(
            "║ \x1B[36m{:4}\x1B[0m ║ {:13} ║ {:13} ║ {:5} ║",
            i + 1,
            user.username,
            mask_password(&user.password),
            role
        );
    }

    // Footer
    println!("\x1B[1;34m╚══════╩═══════════════╩═══════════════╩═══════╝\x1B[0m");
}

// Helper function hash password
fn mask_password(password: &str) -> String {
    if password.is_empty() {
        return String::new();
    }
    "•".repeat(password.len().min(10)) // Show max 10 dots
}

// function untuk handle add users
pub fn handle_users(service: &mut UserService) -> Result<(), UsersError> {
    println!("====================");
    println!("== Tambah User ==");
    println!("====================");
    println!();

    let username = input_users::get_username()?;
    let password = input_users::get_password()?;
    let role = input_users::get_role()?;

    service.add_user(username, password, role)?;

    println!("User berhasil ditambahkan!");
    Ok(())
}
