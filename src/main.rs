use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::bson::doc;
use std::env;

mod db;
mod handler;
mod model;

use handler::user::{create_user, delete_user_by_email, get_all_users, get_user_by_email, update_name};

pub async fn get_all_user(db: web::Data<mongodb::Database>) -> HttpResponse {
    let collection = get_user_collection(db);
    match collection.find(None).await {
        Ok(mut cursor) => {
            let mut users = Vec::new();
            while let Some(user) = cursor.next().await.unwrap_or(None) {
                users.push(user);
            }
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            eprintln!("Failed to fetch users: {}", err);
            HttpResponse::InternalServerError().json("Error fetching users.")
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from a .env file
    dotenv().ok();
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    println!("Mongo URI: {}", mongo_uri);
    let db = db::init_db().await;
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share database connection with handlers
            .route("/api/users", web::post().to(create_user)) // Route for creating a user
            .route("/api/users", web::get().to(get_all_users)) // Route for fetching all users
            .route("/api/users/{email}", web::get().to(get_user_by_email)) // Route for fetching a user by email
            .route("/api/users/{email}", web::put().to(update_name)) // Route for updating a user's name
            .route("/api/users/{email}", web::delete().to(delete_user_by_email)) // Route for deleting a user
    })
    .bind("127.0.0.1:8080")? // Bind server to localhost on port 8080
    .run()
    .await
}
