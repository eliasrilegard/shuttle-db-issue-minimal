use std::thread;
use std::time::Duration;

type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use tracing::{error, info};

pub struct Database {
  pub(self) pool: PgPool
}

impl Database {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub fn watch_reminders(&self) -> Result<()> {
    let pool = self.pool.clone();

    tokio::spawn(async move {
      thread::sleep(Duration::from_secs(10));

      loop {
        info!("Beginning loop");

        // Comment out this block to not call get_reminders and have the bot behave like normal
        match get_reminders(&pool).await {
          // `reminders` will always be empty (conscious choice, see get_reminders),
          // since it's the control flow that's messing things up
          Ok(reminders) => info!("Found {} reminders", reminders.len()),
          Err(why) => error!("Could not retrieve reminders:\n{}", why)
        }

        thread::sleep(Duration::from_secs(60));
      }
    });

    Ok(())
  }
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(FromRow)]
struct PartialReminder {
  reminder_id: i32,
  due_at: DateTime<Utc>,
  created_at: DateTime<Utc>,
  channel_snowflake: i64,
  message_snowflake: i64,
  reminder_message: Option<String>
}

async fn get_reminders(pool: &PgPool) -> Result<Vec<PartialReminder>> {
  // As far as I can tell, this query request is what is causing the strange behavior.
  // However, I'm not sure what's *actually* causing it
  let partial_reminders = match sqlx::query_as::<_, PartialReminder>(
    "SELECT * FROM reminders WHERE
      due_at::DATE = NOW()::DATE AND
      due_at::TIME < NOW()::TIME + INTERVAL '1 minute'"
  )
  .fetch_all(pool)
  .await {
    Ok(result) => result,
    Err(why) => {
      error!("Error querying:\n{:?}", why);
      vec![]
    }
  };
  
  info!("Result from database:\n{:?}", partial_reminders);
  
  // Just need to return something, doesn't matter what in this case
  Ok(vec![])
}