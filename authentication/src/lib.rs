use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

pub fn login(username: &str, password: &str) -> Option<LoginAccess> {
    let password = hash_password(password);
    let users = get_users();
    if let Some(user) = users.get(username) {
        if user.password == password {
            return Some(LoginAccess::Granted(user.role.clone()));
        } else {
            return Some(LoginAccess::Denied);
        }
    }

    None
}

pub fn register(username: &str, password: &str) {
    let mut users = default_users();
    users.insert(username.to_string(), User::new(username, password, UserRole::User));
    save_user(users);
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    role: UserRole,
}

impl User {
    fn new(username: &str, password: &str, role: UserRole) -> Self {
        Self {
            username: username.to_string(),
            password: hash_password(password),
            role,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
}

pub enum LoginAccess {
    Granted(UserRole),
    Denied,
}

fn default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", UserRole::Admin),
    );
    users.insert(
        "user".to_string(),
        User::new("user", "password", UserRole::User),
    );
    users
}

fn get_users() -> HashMap<String, User> {
    let user_path = Path::new("user.json");

    if user_path.exists() {
        //Read file
        let user_json = std::fs::read_to_string(user_path).unwrap();
        let users:HashMap<String, User> = serde_json::from_str(&user_json).unwrap();
        users
    } else {
        //Write file
        let users = default_users();
        let user_json = serde_json::to_string(&users).unwrap();
        std::fs::write(user_path, user_json).expect("error");

        users
    }
}

pub fn delete_user(username: &str) {
    let mut users = default_users();
    users.remove_entry(username);
    save_user(users);
}

pub fn change_password(username: &str, password: &str) {
    let mut users = default_users();
    if let Some(user) = users.get_mut(username) {
        user.password = hash_password(password);
        println!("Password Successfuly updated")
    } else {
        println!("user not found")
    }
    save_user(users);
}

fn save_user(user: HashMap<String, User>) {
    let user_path = Path::new("user.json");
    let user_json = serde_json::to_string(&user).unwrap();
    fs::write(user_path, user_json).unwrap();
}

fn hash_password(password: &str) -> String {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
