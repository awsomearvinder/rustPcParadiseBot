use serenity::prelude::{EventHandler, Context};
use serenity::model::channel::Message;

/// A Cog to help manage the Valheim server.
pub struct ServerManagement;

impl ServerManagement {
    /// A method that just restarts the valheim service using systemctl.
    #[inline(always)]
    async fn restart(&self, ctx: &Context, msg: &Message) {
        let msg = msg.reply(ctx, "restarting...").await;
        let mut restart_valheim_cmd = tokio::process::Command::new("systemctl");
        let output = restart_valheim_cmd
            .arg("restart")
            .arg("valheim")
            .output()
            .await;

        match (msg, output) {
            (Ok(mut m), Ok(_)) => {
                m.edit(ctx,|m| m.content("Server restarted!"))
                    .await
                    .unwrap_or_else(|e|eprintln!("[ ERROR ] Failed to edit message:\n{}", e));
            },
            (Ok(mut m), Err(e)) => {
                eprintln!("[ ERROR ] Failed to restart valheim service:\n{} ", e);
                m.edit(ctx,|m| m.content(format!("[ ERROR ] Failed to restart server with error:\n{}", &e)))
                    .await
                    .unwrap_or_else(|e|eprintln!("[ ERROR ] Failed to edit message:\n{}", e));
            },
            (Err(e), Ok(_)) => {
                eprintln!("[ ERROR ] Failed to send message with error:\n{}", e)
            },
            (Err(e), Err(e2)) => {
                eprintln!("[ ERROR ] Failed to restart valheim service:\n{} ", e);
                eprintln!("[ ERROR ] failed to send message:\n{}", e2);
            }
        }
    }
}


#[async_trait::async_trait]
impl EventHandler for ServerManagement {
    async fn message(&self, ctx: Context, msg: Message ) {
        match msg.content.trim() {
            "%restart" => {
                self.restart(&ctx, &msg).await;
            }
            _ => ()
        }
    }
}
