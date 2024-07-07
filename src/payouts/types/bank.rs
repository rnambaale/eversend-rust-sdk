use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bank {

    pub active: bool,

    pub branch: Branch,

    pub id: String,

    pub name: String,
}

#[derive(Deserialize)]
pub struct Branch {

    pub city: String,

    pub code: String,

    pub id: String,

    pub name: String,

    pub state: String,
}
