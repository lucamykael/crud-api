use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AllUsers {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUser {
    pub name: String,
    pub email: String,
    pub password: String
}