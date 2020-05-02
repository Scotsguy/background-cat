use lazy_static::lazy_static;
use log::{debug, error, info};
use regex::Regex;
use reqwest::get;
use std::{collections::HashSet, env};

use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
    utils::Colour,
};

mod parsers;
use parsers::PARSERS;

mod commands;
use commands::{FUN_GROUP, OTHER_GROUP, STATICIMAGE_GROUP, STATICTEXT_GROUP};

mod hook;
use hook::after_hook;

#[tokio::main]
async fn main() {
    kankyo::load(false).expect("expected a .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("expected a token in $DISCORD_TOKEN");

    let http = Http::new_with_token(&token);
    let bot_id = http
        .get_current_application_info() // what a mouthful
        .await
        .expect("couldn't get info on the bot user")
        .id;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix(&env::var("BACKGROUND_CAT_PREFIX").unwrap_or_else(|_| "-".to_string()))
                .case_insensitivity(true)
        })
        .group(&STATICTEXT_GROUP)
        .group(&STATICIMAGE_GROUP)
        .group(&FUN_GROUP)
        .group(&OTHER_GROUP)
        .help(&MY_HELP)
        .after(after_hook);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES)
        .await
        .expect("error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

#[help]
#[strikethrough_commands_tip_in_guild(" ")]
#[strikethrough_commands_tip_in_dm(" ")]
#[individual_command_tip = " "]
#[max_levenshtein_distance(3)]
#[embed_success_colour(DARK_TEAL)]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await
}

fn common_mistakes(input: &str) -> Vec<(&str, String)> {
    PARSERS.iter().flat_map(|m| m(input)).collect()
}

/// Takes a string of an URL, returns the content.
/// Helper for Error Handling.
async fn get_log(link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let link: reqwest::Url = link.parse()?;
    Ok(get(link).await?.text().await?)
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        lazy_static! {
            static ref PASTEE_REGEX: Regex = Regex::new(r"https:/{2}paste.ee/p/[^\s/]+").unwrap();
        }

        if let Some(link) = PASTEE_REGEX.find(&msg.content) {
            info!(
                "Found paste.ee link: {} in message {}",
                link.as_str(),
                msg.id
            );

            let link_raw = link.as_str().replacen("/p/", "/r/", 1);
            let log = match get_log(&link_raw).await {
                Ok(o) => o,
                Err(_) => return,
            };
            debug!("Content of log: {}", log);

            let mistakes = common_mistakes(&log);

            if mistakes.is_empty() {
                info!("Didn't find any mistakes in log ({})", link.as_str());
                return;
            }
            debug!("Mistakes found: {:?}", mistakes);

            if let Err(why) =
                msg.channel_id
                    .send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Automated Response: (Warning: Experimental)");
                            e.colour(Colour::DARK_TEAL);
                            for i in mistakes.iter() {
                                e.field(i.0, &i.1, true);
                            }
                            e.footer(|f| {
                        f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1");
                        f.text("This might not solve your problem, but it could be worth a try")
                    });
                            debug!("Embed: {:?}", e);
                            e
                        });
                        debug!("Embed: {:?}", m);
                        m
                    })
                    .await
            {
                error!("Couldn't send message: {}", why)
            }
            return;
        };
    }

    // TODO: delete on reaction

    async fn ready(&self, ctx: Context, ready: Ready) {
        use serenity::model::{gateway::Activity, user::OnlineStatus};

        info!("{} is connected!", ready.user.tag());
        ctx.set_presence(
            Some(Activity::playing("DM me: -info")),
            OnlineStatus::Online,
        )
        .await;
    }
}
