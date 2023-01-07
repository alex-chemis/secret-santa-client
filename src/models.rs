use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct NewUser {
    pub name: String
}

#[derive(Serialize, Debug)]
pub struct UpdatedUser {
    pub name: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub is_close: bool,
}

#[derive(Serialize, Debug)]
pub struct NewGroup {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub is_admin: bool,
}

#[derive(Deserialize, Debug)]
pub struct PartialMember {
    pub id: i32,
    pub group_id: i32,
    pub is_admin: bool,
}

#[derive(Deserialize, Debug)]
pub struct NamedMember {
    pub id: i32,
    pub name: String,
    pub group_id: i32,
    pub is_admin: bool,
}

