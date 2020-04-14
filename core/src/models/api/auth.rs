#[derive(Debug, Serialize, Deserialize)]
pub struct LoginIn {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginOut {
    pub jwt_token: String,
}
