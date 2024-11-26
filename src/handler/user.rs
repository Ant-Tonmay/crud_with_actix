use actix_web::{web, HttpRequest, HttpResponse};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::model::User;

// Utility function to get the MongoDB collection
fn get_user_collection(db: web::Data<mongodb::Database>) -> Collection<User> {
    db.collection::<User>("users")
}

// Create a new user
pub async fn create_user(
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    user: web::Json<User>,
) -> HttpResponse {
    let collection = get_user_collection(db);
    let new_user = User {
        id: None,
        name: user.name.clone(),
        email: user.email.clone(),
    };
    match collection.insert_one(&new_user).await {
        Ok(inserted) => {
            if let Some(_id) = inserted.inserted_id.as_object_id() {
                HttpResponse::Ok().json(new_user)
            } else {
                HttpResponse::InternalServerError().json("Failed to retrieve inserted ObjectId.")
            }
        }
        Err(err) => {
            eprintln!("Failed to insert new user: {}", err);
            HttpResponse::InternalServerError().json("Error inserting new user.")
        }
    }
}

// Get all users
pub async fn get_all_users(db: web::Data<mongodb::Database>) -> HttpResponse {
    let collection = get_user_collection(db);
    
    match collection.find(doc! {"id":""}).await {
        Ok(mut cursor) => {
            let mut users = Vec::new();
            
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(user) => users.push(user),
                    Err(err) => {
                        eprintln!("Error processing user document: {}", err);
                        return HttpResponse::InternalServerError().json("Error processing user documents");
                    }
                }
            }
            
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            eprintln!("Failed to fetch users: {}", err);
            HttpResponse::InternalServerError().json("Error fetching users")
        }
    }
}

// Get a user by email
pub async fn get_user_by_email(
    db: web::Data<mongodb::Database>,
    email: web::Path<String>,
) -> HttpResponse {
    let collection = get_user_collection(db);
    let filter = doc! { "email": email.to_string() };
    match collection.find_one(filter).await {
        Some(Ok(user)) => HttpResponse::Ok().json(user),
        Some(Err(err)) => {
            eprintln!("Failed to fetch user by email: {}", err);
            HttpResponse::InternalServerError().json("Error fetching user by email.")
        }
        None => HttpResponse::NotFound().json("User not found."),
    }
}

// Update a user's name
pub async fn update_name(
    db: web::Data<mongodb::Database>,
    email: web::Path<String>,
    new_name: web::Json<String>,
) -> HttpResponse {
    let collection = get_user_collection(db);
    let filter = doc! { "email": email.to_string() };
    let update = doc! { "$set": { "name": new_name.clone() } };
    match collection.update_one(filter, update).await {
        Ok(result) => {
            if result.matched_count > 0 {
                HttpResponse::Ok().json("User's name updated successfully.")
            } else {
                HttpResponse::NotFound().json("User not found.")
            }
        }
        Err(err) => {
            eprintln!("Failed to update user's name: {}", err);
            HttpResponse::InternalServerError().json("Error updating user's name.")
        }
    }
}

// Delete a user by email
pub async fn delete_user_by_email(
    db: web::Data<mongodb::Database>,
    email: web::Path<String>,
) -> HttpResponse {
    let collection = get_user_collection(db);
    let filter = doc! { "email": email.to_string() };
    match collection.delete_one(filter).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().json("User deleted successfully.")
            } else {
                HttpResponse::NotFound().json("User not found.")
            }
        }
        Err(err) => {
            eprintln!("Failed to delete user: {}", err);
            HttpResponse::InternalServerError().json("Error deleting user.")
        }
    }
}
