use teloxide::{prelude::*, types::Recipient};
use tracing::{event, Level};

use crate::db::{DbPool, User};

use super::types::HandlerResult;

pub async fn notify_admins(bot: &Bot, db: &DbPool, message: String) -> HandlerResult {
    let admins: Vec<User> = sqlx::query_as(
        r#"SELECT * FROM "user" WHERE is_admin = true AND has_private_chat = true;"#,
    )
    .fetch_all(db)
    .await?;

    for admin in admins {
        let res = bot
            .send_message(Recipient::Id(ChatId(admin.tg_id)), &message)
            .await;
        if let Err(e) = res {
            event!(Level::WARN, "notify admin {} error {}", admin, e);
        }
    }
    Ok(())
}
