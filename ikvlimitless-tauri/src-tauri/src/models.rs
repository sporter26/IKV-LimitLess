use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password: String,
    pub server: String,
    pub server_port: i32,
    pub is_running: bool,
    pub farm_mode: bool,
    pub boss_mode: bool,
    pub created_at: String,
    pub pid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAccountRequest {
    pub username: String,
    pub password: String,
    pub server: String,
    pub server_port: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub id: String,
    pub username: String,
    pub password: String,
    pub server: String,
    pub server_port: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchAllResponse {
    pub data: usize,
}

// LoginWindowRequest is handled inline in commands.rs
