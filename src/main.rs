use anyhow::Context;
use serenity::all::GatewayIntents;
use serenity::Client;
use shuttle_secrets::{SecretStore, Secrets};
use shuttle_serenity::ShuttleSerenity;
use shuttle_shared_db::Postgres;
use sqlx::{PgPool, Executor};

mod database;
mod handler;

use database::Database;
use handler::Handler;

#[shuttle_runtime::main]
async fn serenity(#[Postgres] pool: PgPool, #[Secrets] secret_store: SecretStore) -> ShuttleSerenity {
  let token = secret_store.get("DISCORD_TOKEN").context("DISCORD_TOKEN not found")?;
  let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;

  pool.execute(include_str!("../schema.sql"))
    .await
    .context("Failed to run migrations")?;

  let db = Database::new(pool);
  let _ = db.watch_reminders(); // <-- This is where the trouble begins

  let handler = Handler::default();

  let client = Client::builder(token, intents)
    .event_handler(handler)
    .await
    .context("Error creating client")?;
  Ok(client.into())
}
