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
fn create_rustacean() -> Value { // Cria um novo rustacean
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
#[catch(404)] // Define um manipulador de erro para o código de status 404 
fn not_found() -> Value { // Retorna um JSON indicando que o recurso não foi encontrado
    json!({"not found": true}) // Class 7
}

// #[rocket::main] // Define a função principal como assíncrona e inicializa o Rocket
#[rocket::main] 
async fn main() { 
    let _ = rocket::build()// Cria uma nova instância do Rocket
        .mount("/", routes![
            get_rustaceans, // Monta a rota para OBTER todos os rustaceans
            view_rustacean, // Monta a rota para VISUALIZAR um rustacean específico
            create_rustacean, // Monta a rota para CRIAR um novo rustacean
            update_rustacean, // Monta a rota para ATUALIZAR um rustacean existente
            delete_rustacean, // Monta a rota para EXCLUIR um rustacean
        ]) // Monta a rota definida acima
        .register("/", catchers![not_found]) // Registra o manipulador de erro para 404
        .launch()   // Lança o servidor Rocket
        .await;     // Aguarda o término do lançamento do servidor
}
/*
Class 5
#[get("/")] // #[get"/"] // Define uma rota GET para o caminho "/"

fn hello() -> Value {
   json!("Hello, world!")
} */