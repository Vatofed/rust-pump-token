use serde::Serialize;

#[derive(Serialize)]
pub struct Token {
    name: String,
    symbol: String,
    total_supply: u64,
    description: String,
}

pub struct TokenParameters {
    pub name: String,
    pub user_supply: u64,
    pub description: String,
}

pub fn generate_token(params: &TokenParameters) -> Token {
    let symbol = format!("{}{}", &params.name[0..3].to_uppercase(), rand::random::<u32>());
    
    Token {
        name: params.name.clone(),
        symbol,
        total_supply: params.user_supply,
        description: params.description.clone(),
    }
}
