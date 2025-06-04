pub mod gen;
pub mod rng;
pub mod translate;

use crate::gen::LolGen64;
use crate::translate::translate;
use teloxide::prelude::*;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
};

async fn handle_inline_query(
    bot: Bot,
    q: InlineQuery,
) -> ResponseResult<()> {
    log::info!(
        "got inline request #{} from {} @{}, text: {}",
        q.id,
        q.from.id,
        q.from.username.unwrap_or("".into()),
        q.query.clone()
    );

    let input = q.query.clone();
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let fixed = translate(&input, true, seed);

    let result = InlineQueryResultArticle::new(
        LolGen64::new(fixed.as_bytes()).to_string(),
        "Monke lang (ONLY FOR RUSSIAN LANGUAGE)",
        InputMessageContent::Text(InputMessageContentText::new(fixed.clone())),
    )
    .description(fixed);

    bot.answer_inline_query(q.id, vec![InlineQueryResult::Article(result)])
        .cache_time(60 * 60 * 24)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Bot started");

    let bot_token = std::env::var("BOT_TOKEN").expect("no bot token in BOT_TOKEN env var");
    let bot = Bot::new(bot_token);

    let handler = Update::filter_inline_query().branch(dptree::endpoint(handle_inline_query));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
