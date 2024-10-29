mod token_generator;
mod blockchain_interaction;
mod user_interface;

#[tokio::main]
async fn main() {
    println!("Welcome to PumpTokenGenerator!");

    // Handle user input to create a new token
    let token_parameters = user_interface::get_user_input();
    
    // Generate token using AI-driven parameters
    let generated_token = token_generator::generate_token(&token_parameters);
    
    // Interact with Solana blockchain to create the token
    blockchain_interaction::create_token(&generated_token).await.unwrap();
    
    println!("Token created successfully!");
}
