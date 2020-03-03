use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
    utils::Colour,
};

use log::{debug, error};

macro_rules! static_text_command {
    ( $($name:ident $($($aliases:ident)+)?, $title:tt, $message:tt;)+ ) => {
        #[group("Text")]
        #[commands( $($name),* )]
        struct StaticText;

        $(
            #[command]
            $( #[aliases($($aliases),+)] )?
            fn $name(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
                if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title($title);
                        e.colour(Colour::DARK_TEAL);
                        e.field(":question:", $message, false);
                        e.footer(|f| {
                            f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1")
                        })
                    });
                    debug!("Message: {:?}", m);
                    m
                }) {
                    error!("couldn't send message: {}", why);
                }
                Ok(())
            }
        )+
    };
}

macro_rules! static_image_command {
    ( $($name:ident $($($aliases:ident)+)?, $image:tt$(, $message:tt)?;)+ ) => {
        #[group("Images")]
        #[commands( $($name,)* )]
        struct StaticImage;

        $(
            #[command]
            $( #[aliases($($aliases),+)] )?
            fn $name(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
                if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.image($image);
                        $(e.title($message);)?
                        e.colour(Colour::DARK_TEAL);
                        e.footer(|f| {
                            f.icon_url("https://cdn.discordapp.com/emojis/280120125284417536.png?v=1")
                        })
                    });
                    debug!("Message: {:?}", m);
                    m
                }) {
                    error!("couldn't send message: {}", why);
                }
                Ok(())
            }
        )+
    };

}

// Format: Name (Optional Alias1 Alias2...) , Title , Message ;
static_text_command! {
    new_forge bsforge, "Forge in 1.13.2 and above is not supported in MultiMC.",
        "For more info, please read https://multimc.org/posts/forge-114.html";
    install_java ijava, "Please install Java 8:",
        "https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java";
    too_much_ram tmram vazkiiram,
        "Allocating too much RAM to Minecraft is bad for performance:",
        "https://vazkii.net/#blog/ram-explanation";
}

// Format: Name (Optional Alias1 Alias2...) , Image Link (, Optional Message) ;
static_image_command! {
    upload_log log, "https://cdn.discordapp.com/attachments/531598137790562305/575381000398569493/unknown.png",
        "Please upload your log:";
    select_java sjava, "https://cdn.discordapp.com/attachments/531598137790562305/575378380573114378/unknown.png",
        "Please select your Java version in the MultiMC settings:";
    select_memory smemory sram, "https://cdn.discordapp.com/attachments/531598137790562305/575376840173027330/unknown.png",
        "Please set your instance memory allocation:";
}

#[group]
#[commands(info)]
struct Other;

#[command]
fn info(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("<:backgroundcat:280120125284417536>A bot to parse logfiles on the MultiMC discord<:backgroundcat:280120125284417536>");
                    let creator_name = match UserId::from(185_461_862_878_543_872).to_user(&ctx) {
                        Ok(o) => o.tag(),
                        Err(why) => {error!("Couldn't get info about creator: {}", why); "<Error getting name>".to_string()}
                    };
                    e.colour(Colour::DARK_TEAL);
                    e.description(format!(r"
Developed by {}.
To start, just upload a log from MultiMC. (Type `-log` for help)

[Source Code available under AGPLv3](https://gitlab.com/Scotsguy/background-cat)
", creator_name))
                });
                m
            }) {
                error!("Couldn't send info message: {}", why)
            }
    Ok(())
}
