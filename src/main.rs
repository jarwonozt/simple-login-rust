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

    let is_valid = utilities::password::is_valid(&password, &hased);
    println!("Status : {is_valid}");
}
