mod docker_controller;
mod models;
mod system_controller;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        docker_controller::get_containers,
        docker_controller::start_container,
        docker_controller::stop_container,
        docker_controller::restart_container,
        docker_controller::get_container_logs,
        system_controller::system_uptime,
        system_controller::system_cpu_load,
        system_controller::system_memory,
    ),
    components(schemas(
        models::Container,
        models::DockerResponse,
        models::SystemResponse
    ))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/docker", docker_controller::docker_routes())
        .nest("/system", system_controller::system_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
