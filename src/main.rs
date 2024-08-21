// Importa os módulos nescessários do Actix-web, que é usado para construir a API.
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
// Importa o cors
use actix_cors::Cors;
// Importa os macros e traits do Serde para serializar e deserializar os dados.
use serde::{Deserialize, Serialize};
// Importa  a conexão com o banco de dados SQLite usando o SQLx.
use sqlx::{SqlitePool};
// Importa o módulo para lidar com variáveis de ambiente.
use std::env;
// Importa o modulo dotenv para carregar as  variaveis de ambiente de um arquivo .env.
use dotenv::dotenv;


// Define a estrutura de dados Todo, que representa uma tarefa na lista de afazeres.
// A estrutura é serializavél e deserializavél, permitindo que ela seja convertida de/para JSON.
#[derive(Serialize, Deserialize)]
struct Todo{
    id: i64,            // Identificador.
    title: String,      // Titulo ou descrição da tarefa.
    completed: bool,    // Estado de conclusão da tarefa. (true = concluida, false= não concluida).
}

// Função assincrona que retorna todas as tarefas (todos) do banco de dados.
// Ela recebe uma conexão do pool de banco de dados.
async fn get_todos(pool: web::Data<SqlitePool>) -> impl Responder{
    // Executa a consulta SQL para selecionar todas as tarefas da tabela 'todos'.
    // O resultado é mapeado para a estrutura Todo.
    let todos = sqlx::query_as!(Todo, "SELECT id, title, completed FROM todos")
        .fetch_all(pool.get_ref())      // Recupera todas as linhas da consulta.
        .await                          // Espera a operação assincrona completar.
        .unwrap();                       // Trata erros (nesse caso, usa unwrap para panicar se houver erro).


        // Retorna a resposta HTTP 200 (OK) com os dados das tarefas no formato JSON. 
        HttpResponse::Ok().json(todos)
}

    // Define uma estrutura para os dados recebidos ao criar uma nova tarefa.
#[derive(Deserialize)]
struct CreateTodo{
    title: String,   // Título da nova tarefa.
}

    // Função assincrona que insere uma nova tarefa no banco de dados.
async fn create_todo(pool: web::Data<SqlitePool>, new_todo: web::Json<CreateTodo>) -> impl Responder{
        // Executa a consulta SQL para inserir uma nova tarefa na tabela 'todos'.
    sqlx::query!("INSERT INTO todos (title, completed) VALUES (?, ?)", new_todo.title, false)
        .execute(pool.get_ref())        // Executa a consulta no banco de dados.
        .await                          // Espera a operação assincrona completar.
        .unwrap();                      // Trata erros (usa unwrap para panicar se houver erro).


        // Retorna a resposta HTTP 201 (Created) indicando que a tarefa foi criada com sucesso.
    HttpResponse::Created().finish()
}

    // Define uma estrutura para os dados recebidos ao atualizar o estado de uma tarefa.
#[derive(Deserialize)]
struct UpdateTodo{
    completed: bool,       // Novo estado de conclusão da tarefa (true ou false).
}

    // Função assincrona que atualiza o estado de uma tarefa no banco de dados.
async fn update_todo(pool: web::Data<SqlitePool>, todo_id: web::Path<i32>, update: web::Json<UpdateTodo>) -> impl Responder{
    // Executa a consulta SQL para atualizar o estado da tarefa na tabela 'todos'.
    sqlx::query!("UPDATE todos SET completed = ? WHERE id = ?", update.completed, *todo_id)
        .execute(pool.get_ref())        // Executa a consulta no banco de dados.
        .await                          // Espera a operação assincrona completar.
        .unwrap();                      // Trata erros (usa unwrap para panicar se houver erro).

        // Retorna a resposta HTTP 200 (Ok) indicando que a tarefa foi atualizada com sucesso.
    HttpResponse::Ok().finish()
} 

    // Função assincrona que deleta uma tarefa do banco de dados.
async fn delete_todo(pool: web::Data<SqlitePool>, todo_id: web::Path<i32>) -> impl Responder{
        // Executa a consulta SQL para deletar a tarefa da tabela 'todos'.
        sqlx::query!("DELETE FROM todos WHERE id = ?", *todo_id)
        .execute(pool.get_ref())        // Executa a consulta no banco de dados.
        .await                          // Espera a operação assincrona completar.
        .unwrap();                      // Trata erros (usa unwrap para panicar se houver erro).


    // Retorna a resposta HTTP 200 (Ok) inidicando que a tarefa foi deletada com sucesso.
    HttpResponse::Ok().finish()
}


    // Fução principal que configura e inica o servidor web.
#[actix_web::main]                          // Macro que marca esta função como ponto de entrada do Actix-web.
async fn main() -> std::io::Result<()>{
    dotenv().ok();                // Carrega variaveis de ambiente do arquivo .env, se existir.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");     // Obtem a URL do banco de dados da variavel de ambiente.
    let pool = SqlitePool::connect(&database_url).await.unwrap();           // Conecta ao banco de dados SQLite e cria um pool de conexões.

    // Cria a tabela 'todos' se ela não existir.
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS todos(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        completed BOOLEAN NOT NULL
        )"
    )
    .execute(&pool)         // Executa a consulta SQL para criar a tabela.
    .await                  // Espera a operação assincrona completar.
    .expect("Falied to createtable.");   // Panica se a tabela não puder ser criada.
    println!("Table created successfully."); // Exibe uma menssagem de sucesso na criação da tabela.

    // Configura e inicia o servidor HTTP.
    HttpServer::new(move ||{

        let cors = Cors::default()
        .allow_any_origin() // Permite qualquer origem (pode especificar origens específicas se necessário)
        .allow_any_method() // Permite qualquer método HTTP (GET, POST, etc.)
        .allow_any_header(); // Permite qualquer cabeçalho

        App::new()
        .wrap(cors) // Aplica o middleware CORS
        .app_data(web::Data::new(pool.clone()))     // Disponibiliza o pool de conexões para as rotas.
        .route("/todos", web::get().to(get_todos))      // Define a rota GET /todos para listar todas as tarefas.
        .route("/todos", web::post().to(create_todo))   // Define a rota POST /todos para criar uma nova tarefa.
        .route("todos/{id}", web::put().to(update_todo))     // Define a tona PUT /todos{id} para atualizar uma nova tarefa especifica.
        .route("todos/{id}", web::delete().to(delete_todo))  // Define a rota DELETE /todos/{id} para deletar uma tarefa especifica.
    })
    .bind("127.0.0.1:9000")?               // Configura o servidor para escutar na porta 9000.
    .run()                                 // Inicia o servidor.
    .await                                 // Espera o servidor rodar indefinidamente.
}
