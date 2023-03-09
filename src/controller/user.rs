use actix_web::{get, put, post, delete, web, HttpResponse};
use futures::stream::StreamExt;
use mongodb::{bson::{doc, oid::ObjectId, to_bson}, Client, Collection};

use crate::model::user::User;

const DB_NAME: &str = "teste";
const COLL_NAME: &str = "users";

#[post("/")]
async fn add_user(client: web::Data<Client>, form: web::Json<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/")]
async fn get_all_user(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let mut cursor = collection.find(None, None).await.expect("Error: not being able to get data from database");

    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => results.push(document),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
        }
    }

    HttpResponse::Ok().json(results)
}

#[get("/{id}")]
async fn get_user(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let id = id.into_inner();
    let object_id = ObjectId::parse_str(&id).expect("invalid id");
    let result = collection.find_one(doc! {"_id": &object_id}, None).await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with id {id}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{id}")]
async fn remove_user(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let object_id = ObjectId::parse_str(id.to_string()).expect("invalid id");
    let result = collection.find_one_and_delete(doc! { "_id": object_id }, None).await;

    match result {
        Ok(Some(response)) => HttpResponse::Ok().json(response),
        Ok(None) => HttpResponse::NotFound().body("user not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[put("/{id}")]
async fn update_user(client: web::Data<Client>, id: web::Path<String>, body: web::Json<User>) -> HttpResponse {
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let object_id = ObjectId::parse_str(id.to_string()).expect("invalid id");

    let user_bson = to_bson(&User { ..body.into_inner() }).expect("error");
    let update = doc! { "$set" : user_bson };

    let result = collection.find_one_and_update(doc! { "_id": object_id }, update, None).await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body("No user found")
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(add_user)
        .service(get_all_user)
        .service(get_user)
        .service(update_user)
        .service(remove_user);

    conf.service(scope);
}

