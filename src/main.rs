#[macro_use] extern crate rocket; // Class 6

// Importa o módulo JSON do Rocket para trabalhar com dados JSON
use rocket::serde::json::{Value, json};
use rocket::response::status;


#[get("/rustaceans")]
fn get_rustaceans() -> Value { // Define uma rota GET para o caminho "/rustaceans"
    json!([{"id": 1, "name": "John Doe" }, {"id": 2, "name": "Jane Doe again"}])
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "John Doe","email": "John@doe.com" })
}
#[post("/rustaceans", format = "json")] // Define uma rota POST para o caminho "/rustaceans"
fn crate_rustacean() -> Value { // Cria um novo rustacean
    json!({"id": 3, "name": "Sandro Reis", "email": "John@doe.com"})
} 
#[put("/rustaceans/<id>", format = "json")] // Rota para atualizar um rustacean existente
fn update_rustacean(id: i32) -> Value { // Atualiza um rustacean existente
    json!({"id": id, "name": "John Doe", "email": "John@doe.com"})
}
#[delete("/rustaceans/<_id>")] // Underline no id para indicar que não será usado agora
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

// #[rocket::main] // Define a função principal como assíncrona e inicializa o Rocket
#[rocket::main] 
async fn main() { 
    let _ = rocket::build()// Cria uma nova instância do Rocket
        .mount("/", routes![
            get_rustaceans, // Monta a rota para obter todos os rustaceans
            view_rustacean, // Monta a rota para visualizar um rustacean específico
            crate_rustacean, // Monta a rota para criar um novo rustacean
            update_rustacean, // Monta a rota para atualizar um rustacean existente
            delete_rustacean, // Monta a rota para excluir um rustacean
        ]) // Monta a rota definida acima
        .launch()   // Lança o servidor Rocket
        .await;     // Aguarda o término do lançamento do servidor
}
/*
Class 5
#[get("/")] // #[get"/"] // Define uma rota GET para o caminho "/"

fn hello() -> Value {
   json!("Hello, world!")
} */