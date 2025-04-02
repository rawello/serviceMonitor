use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use deadpool_postgres::{Manager, Pool, PoolConfig};
use serde_json::{json, Value};
use tokio_postgres::{Config, NoTls, Row};

use crate::models::{SellerResponse, TaskRequest, TaskResponse, TaskTypeResponse};

#[derive(Clone)]
struct DbState {
    ozon_pool: Pool,
    wb_pool: Pool,
}

pub fn task_routes() -> Router {
    let db_state = DbState {
        ozon_pool: create_pool("ozonserver"),
        wb_pool: create_pool("wbserver"),
    };

    Router::new()
        .route("/sellers", get(get_sellers))
        .route("/types", get(get_task_types))
        .route("/", get(get_tasks))
        .route("/", post(create_task))
        .route("/{id}", put(update_task))
        .route("/{id}", delete(delete_task))
        .with_state(db_state)
}

fn create_pool(db_name: &str) -> Pool {
    let mut cfg = Config::new();
    cfg.host("postgres");
    cfg.port(5432);
    cfg.user("postgres");
    cfg.password("123321");
    cfg.dbname(db_name);

    let mgr = Manager::new(cfg, NoTls);
    Pool::builder(mgr)
        .config(PoolConfig::new(10))
        .build()
        .unwrap()
}

#[utoipa::path(
    get,
    path = "/tasks/sellers",
    responses(
        (status = 200, description = "List of sellers", body = Vec<SellerResponse>),
        (status = 500, description = "Database error")
    )
)]
async fn get_sellers(State(state): State<DbState>) -> Result<Json<Value>, StatusCode> {
    let mut combined = Vec::new();

    let client = state.ozon_pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let rows = client.query("SELECT seller_id, seller_name FROM seller", &[])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for row in rows {
        combined.push(SellerResponse {
            id: row.get("seller_id"),
            name: row.get("seller_name"),
            platform: "OZON".into(),
        });
    }

    let client = state.wb_pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let rows = client.query("SELECT seller_id, seller_name FROM seller", &[])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for row in rows {
        combined.push(SellerResponse {
            id: row.get("seller_id"),
            name: row.get("seller_name"),
            platform: "WB".into(),
        });
    }

    Ok(Json(json!(combined)))
}

#[utoipa::path(
    get,
    path = "/tasks/types",
    responses(
        (status = 200, description = "List of task types", body = Vec<TaskTypeResponse>),
        (status = 500, description = "Database error")
    )
)]
async fn get_task_types(State(state): State<DbState>) -> Result<Json<Value>, StatusCode> {
    let mut combined = Vec::new();

    let client = state.ozon_pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let rows = client.query("SELECT id, name FROM task_type", &[])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    for row in rows {
        combined.push(TaskTypeResponse {
            id: row.get("id"),
            name: row.get("name"),
            platform: "OZON".into(),
        });
    }

    let client = state.wb_pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let rows = client.query("SELECT id, name FROM task_type", &[])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    for row in rows {
        combined.push(TaskTypeResponse {
            id: row.get("id"),
            name: row.get("name"),
            platform: "WB".into(),
        });
    }

    Ok(Json(json!(combined)))
}

#[utoipa::path(
    get,
    path = "/tasks",
    params(
        ("platform" = Option<String>, Query, description = "Platform filter (OZON/WB)")
    ),
    responses(
        (status = 200, description = "List of tasks", body = Vec<TaskResponse>),
        (status = 500, description = "Database error")
    )
)]
async fn get_tasks(
    State(state): State<DbState>,
    Query(params): Query<Vec<(String, String)>>,
) -> Result<Json<Value>, StatusCode> {
    let platform_filter = params.iter()
        .find(|(k, _)| k == "platform")
        .map(|(_, v)| v.as_str());

    let mut tasks = Vec::new();

    if platform_filter.is_none() || platform_filter == Some("OZON") {
        let client = state.ozon_pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let rows = client.query(
            "SELECT t.id, t.enabled, t.last_date_from, t.load_interval_cron,
                    t.seller_id, t.task_type_id, tt.name as task_type_name
             FROM task_organizer t
             JOIN task_type tt ON t.task_type_id = tt.id", &[]
        ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        process_rows(rows, &mut tasks, "OZON");
    }

    if platform_filter.is_none() || platform_filter == Some("WB") {
        let client = state.wb_pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let rows = client.query(
            "SELECT t.id, t.enabled, t.last_date_from, t.load_interval_cron,
                    t.seller_id, t.task_type_id, tt.name as task_type_name
             FROM task_organizer t
             JOIN task_type tt ON t.task_type_id = tt.id", &[]
        ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        process_rows(rows, &mut tasks, "WB");
    }

    Ok(Json(json!(tasks)))
}

fn process_rows(rows: Vec<Row>, tasks: &mut Vec<TaskResponse>, platform: &str) {
    for row in rows {
        tasks.push(TaskResponse {
            id: row.get("id"),
            enabled: row.get("enabled"),
            last_date_from: row.get("last_date_from"),
            load_interval_cron: row.get("load_interval_cron"),
            seller_id: row.get("seller_id"),
            task_type_id: row.get("task_type_id"),
            task_type_name: row.get("task_type_name"),
            platform: platform.to_string(),
        });
    }
}

#[utoipa::path(
    post,
    path = "/tasks",
    request_body = TaskRequest,
    responses(
        (status = 200, description = "Task created", body = Value),
        (status = 400, description = "Invalid platform"),
        (status = 500, description = "Database error")
    )
)]
async fn create_task(
    State(state): State<DbState>,
    Json(payload): Json<TaskRequest>,
) -> Result<Json<Value>, StatusCode> {
    let pool = match payload.platform.as_str() {
        "OZON" => &state.ozon_pool,
        "WB" => &state.wb_pool,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let client = pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = client.query_one(
        "INSERT INTO task_organizer
         (enabled, last_date_from, load_interval_cron, seller_id, task_type_id)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id",
        &[
            &payload.enabled.unwrap_or(true),
            &payload.last_date_from,
            &payload.load_interval_cron,
            &payload.seller_id,
            &payload.task_type_id,
        ]
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "id": row.get::<_, i64>("id"),
        "platform": payload.platform
    })))
}

#[utoipa::path(
    put,
    path = "/tasks/{platform}/{id}",
    params(
        ("platform" = String, Path, description = "Platform name"),
        ("id" = i64, Path, description = "Task ID")
    ),
    request_body = TaskRequest,
    responses(
        (status = 200, description = "Task updated"),
        (status = 400, description = "Invalid platform"),
        (status = 500, description = "Database error")
    )
)]
async fn update_task(
    State(state): State<DbState>,
    Path((platform, id)): Path<(String, i64)>,
    Json(payload): Json<TaskRequest>,
) -> Result<Json<Value>, StatusCode> {
    let pool = match platform.as_str() {
        "OZON" => &state.ozon_pool,
        "WB" => &state.wb_pool,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let client = pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    client.execute(
        "UPDATE task_organizer SET
            enabled = COALESCE($1, enabled),
            last_date_from = COALESCE($2, last_date_from),
            load_interval_cron = COALESCE($3, load_interval_cron),
            seller_id = COALESCE($4, seller_id),
            task_type_id = COALESCE($5, task_type_id)
         WHERE id = $6",
        &[
            &payload.enabled,
            &payload.last_date_from,
            &payload.load_interval_cron,
            &payload.seller_id,
            &payload.task_type_id,
            &id,
        ]
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"status": "updated"})))
}

#[utoipa::path(
    delete,
    path = "/tasks/{platform}/{id}",
    params(
        ("platform" = String, Path, description = "Platform name"),
        ("id" = i64, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task deleted"),
        (status = 400, description = "Invalid platform"),
        (status = 500, description = "Database error")
    )
)]
async fn delete_task(
    State(state): State<DbState>,
    Path((platform, id)): Path<(String, i64)>,
) -> Result<Json<Value>, StatusCode> {
    let pool = match platform.as_str() {
        "OZON" => &state.ozon_pool,
        "WB" => &state.wb_pool,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let client = pool.get().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    client.execute(
        "DELETE FROM task_organizer WHERE id = $1",
        &[&id]
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"status": "deleted"})))
}