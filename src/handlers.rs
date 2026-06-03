use axum::{Json, http::StatusCode};

use crate::dto::{CreateTransferRequest, CreateTransferResponse};

pub async fn create_transfer(
    Json(req): Json<CreateTransferRequest>,
) -> (StatusCode, Json<CreateTransferResponse>) {
    if let Err(err) = req.validate() {
        let body = CreateTransferResponse {
            status: "error".to_string(),
            transfer_id: None,
            error: Some(format!("{:?}", err)),
            retryable: Some(err.is_retryable()),
        };
        return (err.http_status(), Json(body));
    }
    let transfer_id = format!("{:x}", rand::random::<u128>());
    let body = CreateTransferResponse {
        status: "success".to_string(),
        transfer_id: Some(transfer_id),
        error: None,
        retryable: None,
    };
    (StatusCode::OK, Json(body))
}
