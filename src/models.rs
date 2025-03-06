use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DockerResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SystemResponse {
    pub uptime: String,
    pub cpu_load: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
}
