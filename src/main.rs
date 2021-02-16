use teloxide::{prelude::*, types::ReplyMarkup, utils::command::BotCommand};
use teloxide::types::{KeyboardButton, ReplyKeyboardMarkup};

use finance_bot_tg::entities::company_profile_info::CompanyProfile;

use std::{ops::Deref, sync::{Arc, Mutex}};
use once_cell::sync::Lazy;

static CLIENT: Lazy<Mutex<Arc<reqwest::blocking::Client>>> = Lazy::new(|| {
    Mutex::new(Arc::new(reqwest::blocking::Client::new()))
});

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "open menu.")]
    Menu,
    #[command(description = "get ticker info.")]
    Ticker(String),
}

async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    let url_button = KeyboardButton::new("text".to_string());
    let keyboard = ReplyKeyboardMarkup::default().append_row(vec![url_button]);
    
    let markup = ReplyMarkup::ReplyKeyboardMarkup(keyboard);
    let client = CLIENT.lock().unwrap().deref().clone();

    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Menu => cx.answer("Here you are").reply_markup(markup).send().await?,
        Command::Ticker(ticker) => {
            let info = CompanyProfile::get(client, &ticker, None, None).await;
            match info {
                Ok(e) => cx.answer(e).send().await?,
                Err(_) => cx.answer_str("Invalid ticker").await?
            }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting finance monitoring bot...");

    let bot = Bot::builder().token("1658511181:AAGD8scAIwyhmppJHbxSVge_LiLK8sM4kBA").build();

    let bot_name = "FinanceMonitoring_Bot";
    teloxide::commands_repl(bot, bot_name, answer).await;
}