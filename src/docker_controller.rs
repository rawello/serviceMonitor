use axum::{
    Router,
    routing::{get, post},
};
use bollard::{
    Docker,
    container::{ListContainersOptions, StartContainerOptions, StopContainerOptions},
};
use futures::StreamExt;

#[utoipa::path(
    get,
    path = "/docker/containers",
    responses(
        (status = 200, description = "List of Docker containers", body = String),
        (status = 500, description = "Failed to retrieve containers")
    )
)]
pub async fn get_containers() -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let containers = docker
        .list_containers(None::<ListContainersOptions<String>>)
        .await
        .unwrap();

    let formatted_containers: Vec<_> = containers
        .iter()
        .map(|container| {
            format!(
                "ID: {}, Names: {:?}, Status: {}",
                container.id.as_ref().unwrap(),
                container.names.as_ref().unwrap(),
                container.status.as_ref().unwrap()
            )
        })
        .collect();

    serde_json::to_string(&formatted_containers).unwrap()
}

#[utoipa::path(
    post,
    path = "/docker/start/{id}",
    params(
        ("id" = String, Path, description = "ID of the container to start")
    ),
    responses(
        (status = 200, description = "Container started successfully", body = String),
        (status = 404, description = "Container not found"),
        (status = 500, description = "Failed to start container")
    )
)]
pub async fn start_container(container_id: String) -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    docker
        .start_container::<String>(&container_id, None::<StartContainerOptions<String>>)
        .await
        .unwrap();
    format!("Container {} started", container_id)
}

#[utoipa::path(
    post,
    path = "/docker/stop/{id}",
    params(
        ("id" = String, Path, description = "ID of the container to stop")
    ),
    responses(
        (status = 200, description = "Container stopped successfully", body = String),
        (status = 404, description = "Container not found"),
        (status = 500, description = "Failed to stop container")
    )
)]
pub async fn stop_container(container_id: String) -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    docker
        .stop_container(&container_id, None::<StopContainerOptions>)
        .await
        .unwrap();
    format!("Container {} stopped", container_id)
}

#[utoipa::path(
    post,
    path = "/docker/restart/{id}",
    params(
        ("id" = String, Path, description = "ID of the container to restart")
    ),
    responses(
        (status = 200, description = "Container restarted successfully", body = String),
        (status = 404, description = "Container not found"),
        (status = 500, description = "Failed to restart container")
    )
)]
pub async fn restart_container(container_id: String) -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    docker.restart_container(&container_id, None).await.unwrap();
    format!("Container {} restarted", container_id)
}

#[utoipa::path(
    get,
    path = "/docker/logs/{id}",
    params(
        ("id" = String, Path, description = "ID of the container to retrieve logs")
    ),
    responses(
        (status = 200, description = "Logs retrieved successfully", body = String),
        (status = 404, description = "Container not found"),
        (status = 500, description = "Failed to retrieve logs")
    )
)]
pub async fn get_container_logs(container_id: String) -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let mut logs_stream = docker.logs(
        &container_id,
        Some(bollard::container::LogsOptions::<String> {
            stdout: true,
            stderr: true,
            ..Default::default()
        }),
    );

    let mut log_output = String::new();

    while let Some(log_result) = logs_stream.next().await {
        match log_result {
            Ok(log_chunk) => match log_chunk {
                bollard::container::LogOutput::StdOut { message } => {
                    log_output.push_str(&String::from_utf8_lossy(&message));
                }
                bollard::container::LogOutput::StdErr { message } => {
                    log_output.push_str(&String::from_utf8_lossy(&message));
                }
                _ => {}
            },
            Err(e) => {
                log_output.push_str(&format!("Error reading logs: {}", e));
            }
        }
    }

    log_output
}

pub fn docker_routes() -> Router {
    Router::new()
        .route("/containers", get(get_containers))
        .route("/start/{id}", post(start_container))
        .route("/stop/{id}", post(stop_container))
        .route("/restart/{id}", post(restart_container))
        .route("/logs/{id}", get(get_container_logs))
}
