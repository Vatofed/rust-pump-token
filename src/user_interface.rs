use std::io;

pub struct UserInput {
    pub name: String,
    pub user_supply: u64,
    pub description: String,
}

pub fn get_user_input() -> UserInput {
    let mut name = String::new();
    let mut user_supply = String::new();
    let mut description = String::new();

    println!("Enter the token name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    
    println!("Enter the total supply:");
    io::stdin().read_line(&mut user_supply).expect("Failed to read line");
    
    println!("Enter a brief description:");
    io::stdin().read_line(&mut description).expect("Failed to read line");

    UserInput {
        name: name.trim().to_string(),
        user_supply: user_supply.trim().parse::<u64>().expect("Please enter a valid number"),
        description: description.trim().to_string(),
    }
}
