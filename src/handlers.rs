use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::{
    dto::{CreateTransferRequest, CreateTransferResponse},
    errors::ErrorCode,
    state::AppState,
};

pub async fn post_transfer(
    pool: &PgPool,
    request: &CreateTransferRequest,
) -> Result<String, ErrorCode> {
    let mut tx = pool.begin().await.map_err(|_| ErrorCode::MathError)?;
    let transfer_id = format!("{:x}", rand::random::<u128>());

    sqlx::query("INSERT INTO transfers (id) VALUES ($1)")
        .bind(&transfer_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| ErrorCode::MathError)?;

    let debit_entry_id = format!("{:x}", rand::random::<u128>());
    let credit_entry_id = format!("{:x}", rand::random::<u128>());

    sqlx::query(
        "INSERT INTO ledger_entries (id, transfer_id, account_id, amount, direction) VALUES ($1, $2, $3, $4, $5)",
    ).bind(&debit_entry_id).bind(&transfer_id).bind(&request.from_account_id).bind(&request.amount).bind("Debit").execute(&mut *tx).await.map_err(|_| ErrorCode::MathError)?;

    sqlx::query(
        "INSERT INTO ledger_entries (id, transfer_id, account_id, amount, direction) VALUES ($1, $2, $3, $4, $5)",
    ).bind(&credit_entry_id).bind(&transfer_id).bind(&request.to_account_id).bind(&request.amount).bind("Credit").execute(&mut *tx).await.map_err(|_| ErrorCode::MathError)?;

    tx.commit().await.map_err(|_| ErrorCode::MathError)?;
    Ok(transfer_id)
}

pub async fn create_transfer(
    State(app_state): State<AppState>,
    Json(req): Json<CreateTransferRequest>,
) -> (StatusCode, Json<CreateTransferResponse>) {
    if let Err(err) = req.validate() {
        let body = CreateTransferResponse {
            status: "error".to_string(),
            transfer_id: None,
            error: Some(format!("{:?}", err)),
            retryable: Some(err.is_retryable()),
            idempotency_key: Some(req.idempotency_key),
        };
        return (err.http_status(), Json(body));
    }

    // business validation SKIPPED intentionally

    match post_transfer(&app_state.pool, &req).await {
        Ok(id) => {
            let body = CreateTransferResponse {
                status: "success".to_string(),
                transfer_id: Some(id),
                error: None,
                retryable: None,
                idempotency_key: Some(req.idempotency_key),
            };
            (StatusCode::OK, Json(body))
        }
        Err(err) => {
            let body = CreateTransferResponse {
                status: "failed".to_string(),
                transfer_id: None,
                error: Some(format!("{:?}", err)),
                retryable: Some(err.is_retryable()),
                idempotency_key: Some(req.idempotency_key),
            };
            (err.http_status(), Json(body))
        }
    }
}
