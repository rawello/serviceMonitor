mod docker_controller;
mod models;
mod system_controller;
mod task_controller;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum_prometheus::PrometheusMetricLayer;
use prometheus::{register_int_counter_vec, IntCounterVec};
use lazy_static::lazy_static;

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
        task_controller::get_sellers,
        task_controller::get_tasks,
        task_controller::create_task,
        task_controller::update_task,
        task_controller::delete_task,
        task_controller::get_task_types,
    ),
    components(schemas(
        models::Container,
        models::DockerResponse,
        models::SystemResponse,
        models::SellerResponse,
        models::TaskResponse,
        models::TaskTypeResponse,
        models::TaskRequest,
    ))
)]
struct ApiDoc;

lazy_static! {
    pub static ref REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!(
        "http_requests_total",
        "Total number of HTTP requests",
        &["method", "endpoint"]
    )
    .unwrap();
}
#[tokio::main]
async fn main() {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/docker", docker_controller::docker_routes())
        .nest("/system", system_controller::system_routes())
        .nest("/tasks", task_controller::task_routes())
        .layer(prometheus_layer)
        .route(
            "/metrics",
            axum::routing::get({
                let metric_handle = metric_handle.clone();
                move || async move { metric_handle.render() }
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
    println!("Server started on port 8090");
    axum::serve(listener, app).await.unwrap();
}