
use authentication::{change_password, delete_user, login, register, LoginAccess};
fn main() {
    loop {
        println!("Welcome to our user management system");
        println!("To get started Enter 1 to login and 2 to register:");
        let input = readline();

        if input.parse::<u32>().unwrap() ==  1 {
            println!("Enter your username: ");
        let username = readline();

        println!("Enter your password: ");
        let password = readline();

        match login(&username, &password) {
            Some(LoginAccess::Granted(role)) => {
                match role {
                    authentication::UserRole::Admin => println!("Admin"),
                    authentication::UserRole::User => println!("User"),
                }
                break;
            }

            Some(LoginAccess::Denied) => {
                println!("Denied");
            }

            None => {
                println!("Access Denied")
            }
        }
        }else if  input.parse::<u32>().unwrap() ==  2{
            println!("Enter your username: ");
            let username = readline();
            println!("Enter your password");
            let password = readline();

            register(&username, &password);
            println!("Registration Complete!");
            break;
        } else if input.parse::<u32>().unwrap() ==  2 {
            println!("Enter the username to delete: ");
            let username = readline();

            delete_user(&username);
            println!("User deleted successfully!");
            break;
        } else {
            println!("Enter the username to change password");
            let username = readline();
            println!("Enter new password for: {username}");
            let password = readline();

            change_password(&username, &password);
            break;
        }
        //Getting user credentials
        
    }
}

fn readline() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working correctly");
    input.trim().to_string()
}
