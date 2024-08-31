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
}