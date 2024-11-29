use std::{io, thread, time::Duration};

use utilities::remaining;

mod utilities {
    #[path = "random.rs"]
    pub mod random;

    #[path = "password.rs"]
    pub mod password;

    #[path = "remaining.rs"]
    pub mod remaining;
}

static MAX_LOGIN: u32 = 3; // limit login invalid
static DELAY: u32 = 10; // seconds

fn main() {
    loop {
        let mut count = 0;
        let mut alert = String::new();
        println!("{alert}");

        for _ in 0..MAX_LOGIN {
            count += 1;
            println!("==== SIMPLE LOGIN ====");
            println!();
            let mut username = String::new();
            println!("Enter username : ");

            io::stdin()
                .read_line(&mut username)
                .expect("Invalid username");
            let username = username.trim();

            println!("Username : {username}");

            let password = format!("NOZT-{}", utilities::random::string(10));
            println!("Password Generate : {password}");

            let hased = utilities::password::hash(&password);
            println!("Hased password: {hased}");

            // enter password hashed
            let mut password_input = String::new();
            println!("Enter password generate : ");
            io::stdin()
                .read_line(&mut password_input)
                .expect("Wrong password");

            let is_valid = utilities::password::is_valid(&password_input.trim(), &hased);
            println!();
            if is_valid {
                alert = String::from("Status : Login successfully");
                println!("{alert}");
                break;
            } else {
                alert = String::from("Status : Invalid password, please try again !");
                println!("{alert}");
                println!();
            }
        }

        if count == MAX_LOGIN {
            println!("Limit login, please try again later");
            println!();
            thread::sleep(Duration::from_secs(3));
            let rem = format!("{:?}",remaining::remainings(DELAY));
            println!("{}", rem);
            continue;
        } else {
            break;
        }
    }
}
