use serde::Deserialize;

/// Query String arguments required for viewing a bang list
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(alias = "l")]
    pub list: i32,
    #[serde(alias = "k")]
    pub key: String,
}

/// Query String arguments required for searching using a bang list
#[derive(Deserialize)]
pub struct SearchQuery {
    #[serde(alias = "l")]
    pub list: i32,
    #[serde(alias = "k")]
    pub key: String,
    #[serde(alias = "q")]
    pub query: String,
}

/// Form Data provided to the corresponding route for setting the default search engine
#[derive(Debug, Deserialize)]
pub struct SetBase {
    pub list: i32,
    pub key: String,
    pub url: String,
}

/// Form Data provided to the corresponding route for adding a new bang to the list
#[derive(Debug, Deserialize)]
pub struct AddBang {
    pub list: i32,
    pub key: String,
    pub bang: String,
    pub url: String,
}

/// Form Data provided to the corresponding route for removing a bang from the list
#[derive(Debug, Deserialize)]
pub struct DelBang {
    pub list: i32,
    pub key: String,
    pub bang: String,
}

/// Form Data provided to the corresponding route for editing a bang on the list
#[derive(Debug, Deserialize)]
pub struct EditBang {
    pub list: i32,
    pub key: String,
    pub bang: String,
    pub url: String,
}