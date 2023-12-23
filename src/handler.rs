use serenity::all::{Command, Interaction, Ready};
use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::client::{Context, EventHandler};
use tracing::{error, info};

#[derive(Default)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    info!("Received interaction with id {}", interaction.id());

    match interaction {
      Interaction::Command(command) => {
        match command.data.name.as_str() {
          "ping" => {
            let message = CreateInteractionResponseMessage::new().content("Pong!");
            let response_data = CreateInteractionResponse::Message(message);

            match command.create_response(ctx, response_data).await {
              Ok(_) => info!("Response successful!"),
              Err(why) => error!("{:?}", why)
            }
          }

          _ => error!("Unknown command")
        }
      }

      _ => error!("Unknown interaction")
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    info!("Registering commands...");

    let commands = vec![
      CreateCommand::new("ping").description("Test command")
    ];

    if let Err(why) = Command::set_global_commands(ctx, commands).await {
      error!("Command registration failed:\n{why:?}");
    }

    info!("Ready! Logged in as {}", ready.user.name);
  }
}