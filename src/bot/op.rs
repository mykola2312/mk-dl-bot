use rust_i18n::t;
use sqlx::Row;
use teloxide::prelude::*;
use tracing::{event, Level};

use super::types::HandlerResult;
use crate::db::user::find_or_create_user;
use crate::db::DbPool;

pub async fn cmd_op(bot: Bot, msg: Message, db: DbPool) -> HandlerResult {
    let admins: i64 = sqlx::query(r#"SELECT COUNT(*) FROM "user" WHERE is_admin = true"#)
        .fetch_one(&db)
        .await?
        .get(0);

    if let Some(tg_user) = msg.from() {
        if admins == 0 {
            let user = find_or_create_user(&db, tg_user).await?;
            sqlx::query(r#"UPDATE "user" SET is_admin = true WHERE id = $1;"#)
                .bind(user.id)
                .execute(&db)
                .await?;

            event!(
                Level::INFO,
                "opped {} - {}",
                user.tg_id,
                user.username_or_name()
            );
            bot.send_message(msg.chat.id, t!("op_yourself")).await?;
        } else {
            let user = find_or_create_user(&db, tg_user).await?;
            if user.is_admin {
                if let Some(target) = msg.reply_to_message().and_then(|m| m.from()) {
                    let target = find_or_create_user(&db, target).await?;
                    sqlx::query(r#"UPDATE "user" SET is_admin = 1 WHERE id = $1;"#)
                        .bind(target.id)
                        .execute(&db)
                        .await?;

                    event!(Level::INFO, "opped {}", target);
                    bot.send_message(msg.chat.id, "opped").await?;
                } else {
                    bot.send_message(msg.chat.id, t!("has_to_reply")).await?;
                }
            } else {
                bot.send_message(msg.chat.id, t!("cant_do_that")).await?;
            }
        }
    }

    Ok(())
}
