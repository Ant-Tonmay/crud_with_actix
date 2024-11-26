use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::bson::doc;
use std::env;

mod db;
mod handler;
mod model;

use handler::user::{create_user, delete_user, get_users, get_user, update_user};
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
            .route("/api/users", web::get().to(get_users)) // Route for fetching all users
            .route("/api/users/{email}", web::get().to(get_user)) // Route for fetching a user by email
            .route("/api/users/{email}", web::put().to(update_user)) // Route for updating a user's name
            .route("/api/users/{email}", web::delete().to(delete_user)) // Route for deleting a user
    })
    .bind("127.0.0.1:8080")? // Bind server to localhost on port 8080
    .run()
    .await
}
