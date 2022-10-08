use std::collections::HashMap;

use rocket::{tokio::sync::RwLock, State};

#[macro_use]
extern crate rocket;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Token(pub String);

#[derive(Eq, Hash, PartialEq, Clone)]
struct Password(pub String);

type Tokens = RwLock<HashMap<Token, Password>>;

fn generate_token() -> Token {
    let token = vec![rand::random::<i32>(), rand::random::<i32>(), rand::random::<i32>(),rand::random::<i32>()];
    let token_string: Vec<String> = token.iter().map(|x| x.to_string()).collect();
    Token(token_string.join(""))
}

#[get("/token/<password>")]
async fn token(token_state: &State<Tokens>, password: String) -> String {
    let mut tokens = token_state.write().await;
    let token = generate_token(); 
    tokens.insert(token.clone(), Password(password.clone()));

    token.0
}

#[get("/password/<token>")]
async fn password(token_state: &State<Tokens>, token: String) -> String {
    let tokens = token_state.write().await;
    let password = tokens.get(&Token(token));
    
    if let Some(pwd) = password.cloned() {
        return pwd.0;
    } else {
        return "".into();
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![token, password]).manage(RwLock::new(HashMap::<Token, Password>::new()))
}
