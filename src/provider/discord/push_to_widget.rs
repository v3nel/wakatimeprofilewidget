use crate::{error::AppResult, state::AppState};

pub struct WidgetData {
    pub code_time_total: String,
    pub code_time_7_days: String,
    pub total_lines: String,
    pub ai: String,
    pub human: String,
    pub fav_editor: String,
    pub fav_language: String,
}

pub async fn push_to_widget(state: &AppState, data: WidgetData) -> AppResult<()> {
    let app_id = &state.config.discord_app_id;
    let user_id = &state.config.discord_user_id;
    let bot_token = &state.config.discord_bot_token;

    let body = serde_json::json!({
        "username": "Venel",
        "data": {
            "dynamic": [
                {
                    "type": 1,
                    "name": "coding_time_total",
                    "value": data.code_time_total
                },
                {
                    "type": 1,
                    "name": "coding_time_week",
                    "value": data.code_time_7_days
                },
                {
                    "type": 1,
                    "name": "lines_total",
                    "value": data.total_lines
                },
                {
                    "type": 1,
                    "name": "fav_language",
                    "value": data.fav_language
                },
                {
                    "type": 1,
                    "name": "changes",
                    "value": data.human
                },
                {
                    "type": 1,
                    "name": "ai_changes",
                    "value": data.ai
                },
                {
                    "type": 1,
                    "name": "fav_editor",
                    "value": data.fav_editor
                },
                {
                    "type": 3,
                    "name": "avatar_url",
                    "value": {
                        "url": "https://cdn.discordapp.com/avatars/580447236702470176/198cc3604911c7a6e7083f0efce21899.png?size=1024"
                    }
                }
            ]
        }
    });

    state
        .http
        .patch(format!(
            "https://discord.com/api/v9/applications/{app_id}/users/{user_id}/identities/0/profile"
        ))
        .header("Authorization", format!("Bot {bot_token}"))
        .header(
            "User-Agent",
            "DiscordBot (https://github.com/discord/discord-api-docs, 1.0.0)",
        )
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
