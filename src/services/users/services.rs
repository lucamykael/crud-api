use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use super::models::{AllUsers, DeleteUser, RegisterUser, UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(users) => {
            HttpResponse::Ok().json(
                users
                    .iter()
                    .map(|user| AllUsers {
                        id: user.id,
                        name: user.name.clone(),
                        email: user.email.clone(),
                        password: user.password.clone()
                    })
                    .collect::<Vec<AllUsers>>()
            )
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/users")]
async fn create_user(app_state: web::Data<AppState>, user: web::Json<RegisterUser>) -> impl Responder {
    let hashed = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");

    if !(hashed != user.password){
        return HttpResponse::BadRequest().body("Failed to hash password");
    }

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password",
        user.name,
        user.email,
        hashed
    ).fetch_one(&app_state.postgres_client)
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(AllUsers {
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[put("/users/{id}")]
async fn update_user(app_state: web::Data<AppState>, id: web::Path<i32>, user: web::Json<UpdateUser>) -> impl Responder {
    let hashed = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");

    if !(hashed != user.password){
        return HttpResponse::InternalServerError().body("Failed to sswordupdate password");
    }

    let result = sqlx::query!(
        "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, name, email, password",
        user.name,
        user.email,
        hashed,
        id.into_inner()
    ).fetch_one(&app_state.postgres_client).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(UpdateUser {
            name: user.name,
            email: user.email,
            password: user.password
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1 RETURNING id, name, email, password",
        id.into_inner()
    ).fetch_one(&app_state.postgres_client).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(DeleteUser {
            name: user.name,
            email: user.email,
            password: user.password
        }),
        Err(e) => HttpResponse::NotFound().body(e.to_string())
    }
}

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}