use sqlx::{PgPool, Error};

#[tonic::async_trait]
pub trait PgPoolExt {
  async fn increment_counter(&self) -> Result<i64, Error>;
}

#[tonic::async_trait]
impl PgPoolExt for PgPool {
  async fn increment_counter(&self) -> Result<i64, Error> {
    let rec = sqlx::query!("update counters set value = value + 1 returning value")
      .fetch_one(self)
      .await?;
    Ok(rec.value)
  }
}
