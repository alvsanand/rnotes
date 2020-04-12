#[derive(Serialize, Deserialize)]
pub struct LoginIn {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginOut {
    pub jwt_token: String,
}
