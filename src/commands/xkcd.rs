use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::Colour,
};

use log::error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WikiSearch {
    query: WikiQuery,
}

#[derive(Deserialize, Debug)]
struct WikiQuery {
    search: Vec<WikiQuerySearch>,
}
#[derive(Deserialize, Debug)]
struct WikiQuerySearch {
    title: String,
}

#[command]
#[description = "Fetch an xkcd comic based on its number or title"]
#[usage = "[number or title]"]
#[example = "1987"]
#[example = "standards"]
async fn xkcd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    const XKCD_URL: &str = "https://xkcd.com/";
    const EXPLAINXKCD_API_URL: &str = "https://www.explainxkcd.com/wiki/api.php";
    let number = if args.trimmed().parse::<u32>().is_ok() {
        Some(args.single()?)
    } else {
        let client = reqwest::Client::new();
        let resp = client
            .post(EXPLAINXKCD_API_URL)
            .query(&[
                ("action", "query"),
                ("format", "json"),
                ("list", "search"),
                ("srlimit", "1"),
                ("srwhat", "title"),
                ("srsort", "just_match"),
                ("srsearch", &args.raw().collect::<Vec<&str>>().join(" ")),
            ])
            .send()
            .await?
            .json::<WikiSearch>()
            .await?;

        if let Some(q) = resp.query.search.get(0) {
            if let Some(n) = q.title.split(':').next() {
                if n.parse::<u32>().is_ok() {
                    Some(n.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(number) = number {
        if let Err(why) = msg
            .channel_id
            .say(&ctx, XKCD_URL.to_owned() + &number)
            .await
        {
            error!("Couldn't send xkcd: {}", why);
        };
    } else if let Err(why) = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Couldn't find that comic!");
                e.colour(Colour::RED);
                e.footer(|f| f.text("Search might be improved in the future"))
            })
        })
        .await
    {
        error!("Couldn't send xkcd error message: {}", why);
    }

    Ok(())
}
/*
fn extract_number_from_title<'a>(title: &'a str) -> Option<&'a str> {
    None
}
*/
