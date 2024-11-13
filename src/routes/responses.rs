use serde::Serialize;

#[derive(Serialize)]
pub struct EditResponse {
    pub success: bool,
    pub bang: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub bang: String,
}

#[derive(Serialize)]
pub struct AddResponse {
    pub success: bool,
    pub details: Option<AddResponseDetails>,
}

#[derive(Serialize)]
pub struct AddResponseDetails {
    pub name: String,
    pub url: String,
    pub id: i32,
    pub key: String,
}

#[derive(Serialize)]
pub struct ChangeDefaultResponse {
    pub success: bool,
    pub current: String,
}