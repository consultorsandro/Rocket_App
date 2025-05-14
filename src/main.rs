//#[macro_use] extern crate rocket; // Importa o macro do Rocket para usar macros do framework
#[macro_use] extern crate rocket;

// Importa o módulo JSON do Rocket para trabalhar com dados JSON
use rocket::serde::json::{Value, json}; 

#[get("/")] // #[get"/"] // Define uma rota GET para o caminho "/"

fn hello() -> Value {
   json!("Hello, world!")
}

// #[rocket::main] // Define a função principal como assíncrona e inicializa o Rocket
#[rocket::main] 
async fn main() { 
    let _ = rocket::build()// Cria uma nova instância do Rocket
        .mount("/", routes![hello]) // Monta a rota definida acima
        .launch()   // Lança o servidor Rocket
        .await;     // Aguarda o término do lançamento do servidor
}
