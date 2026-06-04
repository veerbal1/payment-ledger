use rand::distr::{Alphanumeric, SampleString};
use sqlx::PgPool;

pub async fn create_test_account(pool: &PgPool, name: &str) -> String {
    let account_id = Alphanumeric.sample_string(&mut rand::rng(), 16);

    sqlx::query("INSERT INTO accounts (id, name, currency) VALUES ($1, $2, $3)")
        .bind(&account_id)
        .bind(name)
        .bind("USD")
        .execute(pool)
        .await
        .expect("failed to create test account");

    account_id
}
