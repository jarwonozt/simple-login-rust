use std::io;

mod utilities {
    #[path="random.rs"]
    pub mod random;

    #[path="password.rs"]
    pub mod password;
}
fn main() {
    let mut username = String::new();
    println!("Enter username : ");

    io::stdin().read_line(&mut username).expect("Invalid username");
    let username = username.trim();

    println!("Username : {username}");

    let password = format!("NOZT-{}", utilities::random::string(10));
    println!("Password Generate : {password}");

    let hased = utilities::password::hash(&password);
    println!("Hased password: {hased}");

    // enter password hashed
    let mut password_input = String::new();
    println!("Enter password generate : ");
    io::stdin().read_line(&mut password_input).expect("Wrong password");
    // let password_input = password_input.trim();

    let is_valid = utilities::password::is_valid(&password_input.trim(), &hased);
    println!("Status Login : {is_valid}");
}
