use crate::commands::{Context, Result};

/// Show this help 
#[poise::command(prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    command: Option<String>
) -> Result<()>{
    poise::builtins::help(ctx, command.as_deref(), 
        poise::builtins::HelpConfiguration{
            extra_text_at_bottom: "Sample Text",
            ..Default::default()
        }
    ).await?;
    Ok(())
}