use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::{doc, oid::ObjectId}, Collection};
use tokio::sync::futures;
use crate::model::User;
use ::futures_util::TryStreamExt;



fn get_user_collection(db: web::Data<mongodb::Database>) -> Collection<User> {
    db.collection::<User>("users")
}

pub async fn create_user(db: web::Data<mongodb::Database>, user: web::Json<User>) -> impl Responder {
    let collection = get_user_collection(db);
    let new_user = User {
        id: None,
        name: user.name.clone(),
        email: user.email.clone(),
    };
    let result = collection.insert_one(new_user).await;

    match result {
        Ok(inserted) => HttpResponse::Ok().json(inserted.inserted_id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get all users
pub async fn get_users(db: web::Data<mongodb::Database>) -> impl Responder {
    let collection = get_user_collection(db);
    let mut cursor = collection.find(doc! {}).await.unwrap();

    let mut users = vec![];

    while let Some(user) = cursor.try_next().await.unwrap() {
        users.push(user);
    }

    HttpResponse::Ok().json(users)
}
// Get user by ID
pub async fn get_user(db: web::Data<mongodb::Database>, path: web::Path<String>) -> impl Responder {
    let collection = get_user_collection(db);
    // let id = ObjectId::parse_str(&path.into_inner()).unwrap();
    let email = &path.into_inner();

    let user = collection.find_one(doc! { "email": email}).await.unwrap();

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

// Update a user by ID
pub async fn update_user(
    db: web::Data<mongodb::Database>,
    path: web::Path<String>,
    user: web::Json<User>,
) -> impl Responder {
    let collection = get_user_collection(db);
    // let id = ObjectId::parse_str(&path.into_inner()).unwrap();
    let email = &path.into_inner();

    let result = collection
        .update_one(
            doc! { "email": email},
            doc! { "$set": { "name": &user.name}},
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Delete a user by ID
pub async fn delete_user(db: web::Data<mongodb::Database>, path: web::Path<String>) -> impl Responder {
    let collection = get_user_collection(db);
    // let id = ObjectId::parse_str(&path.into_inner()).unwrap();
    let email = &path.into_inner();

    let result = collection.delete_one(doc! { "email": email }).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}