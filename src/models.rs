use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SellerResponse {
    pub id: i64,
    pub name: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskResponse {
    pub id: i64,
    pub enabled: bool,
    #[schema(value_type = String, format = DateTime)]
    pub last_date_from: DateTime<Utc>,
    pub load_interval_cron: String,
    pub seller_id: i64,
    pub task_type_id: i64,
    pub task_type_name: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskTypeResponse {
    pub id: i64,
    pub name: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskRequest {
    pub enabled: Option<bool>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub last_date_from: Option<DateTime<Utc>>,
    pub load_interval_cron: Option<String>,
    pub seller_id: i64,
    pub task_type_id: i64,
    pub platform: String,
}