use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::{
    dto::{CreateTransferRequest, CreateTransferResponse},
    errors::ErrorCode,
    state::AppState,
};

fn is_unique_violation(error: &sqlx::Error) -> bool {
    matches!(
        error,
        sqlx::Error::Database(db_error) if db_error.code().as_deref() == Some("23505")
    )
}

pub async fn post_transfer(
    pool: &PgPool,
    request: &CreateTransferRequest,
    fingerprint: String,
) -> Result<String, ErrorCode> {
    let mut tx = pool.begin().await.map_err(|_| ErrorCode::MathError)?;
    let transfer_id = format!("{:x}", rand::random::<u128>());

    let insert_idempotency_record_result = sqlx::query(
        "INSERT INTO idempotency_records (id, transfer_id, fingerprint, status) VALUES ($1, $2, $3, $4)",
    )
    .bind(&request.idempotency_key)
    .bind(Option::<String>::None)
    .bind(&fingerprint)
    .bind("in_progress")
    .execute(&mut *tx)
    .await;

    if let Err(err) = insert_idempotency_record_result {
        if !is_unique_violation(&err) {
            return Err(ErrorCode::MathError);
        }

        tx.rollback().await.map_err(|_| ErrorCode::MathError)?;

        let (stored_transfer_id, stored_fingerprint) =
            sqlx::query_as::<_, (Option<String>, String)>(
                "SELECT transfer_id, fingerprint FROM idempotency_records WHERE id = $1",
            )
            .bind(&request.idempotency_key)
            .fetch_one(pool)
            .await
            .map_err(|_| ErrorCode::MathError)?;

        if stored_fingerprint == fingerprint {
            if let Some(stored_transfer_id) = stored_transfer_id {
                return Ok(stored_transfer_id);
            }

            return Err(ErrorCode::MathError);
        }

        return Err(ErrorCode::IdempotencyKeyReused);
    }

    sqlx::query("INSERT INTO transfers (id) VALUES ($1)")
        .bind(&transfer_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| ErrorCode::MathError)?;

    let debit_entry_id = format!("{:x}", rand::random::<u128>());
    let credit_entry_id = format!("{:x}", rand::random::<u128>());

    sqlx::query(
        "INSERT INTO ledger_entries (id, transfer_id, account_id, amount, direction) VALUES ($1, $2, $3, $4, $5)",
    ).bind(&debit_entry_id).bind(&transfer_id).bind(&request.from_account_id).bind(request.amount).bind("Debit").execute(&mut *tx).await.map_err(|_| ErrorCode::MathError)?;

    sqlx::query(
        "INSERT INTO ledger_entries (id, transfer_id, account_id, amount, direction) VALUES ($1, $2, $3, $4, $5)",
    ).bind(&credit_entry_id).bind(&transfer_id).bind(&request.to_account_id).bind(request.amount).bind("Credit").execute(&mut *tx).await.map_err(|_| ErrorCode::MathError)?;

    sqlx::query("UPDATE idempotency_records SET transfer_id = $1, status = $2 WHERE id = $3")
        .bind(&transfer_id)
        .bind("completed")
        .bind(&request.idempotency_key)
        .execute(&mut *tx)
        .await
        .map_err(|_| ErrorCode::MathError)?;

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

    match post_transfer(&app_state.pool, &req, req.fingerprint()).await {
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
