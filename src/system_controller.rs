use axum::{Router, routing::get};
use sysinfo::System;

#[utoipa::path(
    get,
    path = "/system/uptime",
    responses(
        (status = 200, description = "System uptime retrieved successfully", body = String),
        (status = 500, description = "Failed to retrieve system uptime")
    )
)]
pub async fn system_uptime() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let uptime = sysinfo::System::uptime();
    format!("{}", uptime)
}

#[utoipa::path(
    get,
    path = "/system/cpu-load",
    responses(
        (status = 200, description = "CPU load retrieved successfully", body = String),
        (status = 500, description = "Failed to retrieve CPU load")
    )
)]
pub async fn system_cpu_load() -> String {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();
    let cpu_usage = sys.global_cpu_usage();
    format!("{:.2}", cpu_usage)
}

#[utoipa::path(
    get,
    path = "/system/memory",
    responses(
        (status = 200, description = "Memory usage retrieved successfully", body = String),
        (status = 500, description = "Failed to retrieve memory usage")
    )
)]
pub async fn system_memory() -> String {
    let mut sys = System::new_all();
    sys.refresh_memory();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;
    format!("{:.2} / {:.2}", used_memory, total_memory)
}

pub fn system_routes() -> Router {
    Router::new()
        .route("/uptime", get(system_uptime))
        .route("/cpu-load", get(system_cpu_load))
        .route("/memory", get(system_memory))
}
