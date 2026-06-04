mod common;

use sqlx::Row;

#[tokio::test]
async fn create_test_account_inserts_account_row() {
    let pool = sqlx::PgPool::connect("postgres://ledger:ledger@localhost:5432/payment_ledger_test")
        .await
        .expect("failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations should apply cleanly");

    let account_id = common::create_test_account(&pool, "Fixture Account").await;

    let row = sqlx::query("SELECT name, currency FROM accounts WHERE id = $1")
        .bind(&account_id)
        .fetch_one(&pool)
        .await
        .expect("test account should exist");

    assert_eq!(row.get::<String, _>("name"), "Fixture Account");
    assert_eq!(row.get::<String, _>("currency"), "USD");
}
