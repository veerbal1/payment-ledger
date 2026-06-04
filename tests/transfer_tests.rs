mod common;

use payment_ledger::{dto::CreateTransferRequest, handlers::post_transfer};
use sqlx::{PgPool, Row};

const TEST_DATABASE_URL: &str = "postgres://ledger:ledger@localhost:5432/payment_ledger_test";
static TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn setup_pool() -> PgPool {
    let pool = PgPool::connect(TEST_DATABASE_URL)
        .await
        .expect("failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations should apply cleanly");

    sqlx::query("DELETE FROM ledger_entries")
        .execute(&pool)
        .await
        .expect("failed to clean ledger entries");
    sqlx::query("DELETE FROM transfers")
        .execute(&pool)
        .await
        .expect("failed to clean transfers");

    pool
}

async fn count_rows(pool: &PgPool, table: &str) -> i64 {
    let sql = match table {
        "transfers" => "SELECT COUNT(*) AS count FROM transfers",
        "ledger_entries" => "SELECT COUNT(*) AS count FROM ledger_entries",
        _ => panic!("unsupported table: {table}"),
    };
    let row = sqlx::query(sql)
        .fetch_one(pool)
        .await
        .expect("failed to count rows");

    row.get("count")
}

#[tokio::test]
async fn post_transfer_inserts_transfer_and_balanced_ledger_entries() {
    let _guard = TEST_LOCK.lock().await;
    let pool = setup_pool().await;
    let from_account_id = common::create_test_account(&pool, "Source Account").await;
    let to_account_id = common::create_test_account(&pool, "Destination Account").await;
    let amount = 500;

    let transfer_id = post_transfer(
        &pool,
        &CreateTransferRequest {
            from_account_id: from_account_id.clone(),
            to_account_id: to_account_id.clone(),
            amount,
        },
    )
    .await
    .expect("transfer should be posted");

    assert_eq!(count_rows(&pool, "transfers").await, 1);
    assert_eq!(count_rows(&pool, "ledger_entries").await, 2);

    let debit_count = sqlx::query(
        "SELECT COUNT(*) AS count FROM ledger_entries
         WHERE transfer_id = $1 AND account_id = $2 AND amount = $3 AND direction = 'Debit'",
    )
    .bind(&transfer_id)
    .bind(&from_account_id)
    .bind(amount)
    .fetch_one(&pool)
    .await
    .expect("failed to count debit entry")
    .get::<i64, _>("count");

    let credit_count = sqlx::query(
        "SELECT COUNT(*) AS count FROM ledger_entries
         WHERE transfer_id = $1 AND account_id = $2 AND amount = $3 AND direction = 'Credit'",
    )
    .bind(&transfer_id)
    .bind(&to_account_id)
    .bind(amount)
    .fetch_one(&pool)
    .await
    .expect("failed to count credit entry")
    .get::<i64, _>("count");

    assert_eq!(debit_count, 1);
    assert_eq!(credit_count, 1);
}

#[tokio::test]
async fn post_transfer_rolls_back_when_credit_account_is_missing() {
    let _guard = TEST_LOCK.lock().await;
    let pool = setup_pool().await;
    let from_account_id = common::create_test_account(&pool, "Source Account").await;

    let result = post_transfer(
        &pool,
        &CreateTransferRequest {
            from_account_id,
            to_account_id: format!("missing-account-{:x}", rand::random::<u128>()),
            amount: 500,
        },
    )
    .await;

    assert!(result.is_err());
    assert_eq!(count_rows(&pool, "transfers").await, 0);
    assert_eq!(count_rows(&pool, "ledger_entries").await, 0);
}
