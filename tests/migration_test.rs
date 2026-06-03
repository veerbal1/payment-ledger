use sqlx;

#[tokio::test]
async fn payment_ledger_test() {
    let pool = sqlx::PgPool::connect("postgres://ledger:ledger@localhost:5432/payment_ledger_test")
        .await
        .expect("failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations should apply cleanly");
}
