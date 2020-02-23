use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
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
            #[only_in("guilds")]
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
            #[only_in("guilds")]
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
    new_forge bsforge, "Forge in 1.13.2 and above is not supported in MultiMC.", "For more info, please read https://multimc.org/posts/forge-114.html";
    install_java ijava, "Please install Java 8:", "https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java";
    too_much_ram tmram vazkiiram, "Too much RAM is bad for performance:", "https://vazkii.net/#blog/ram-explanation";
}

// Format: Name (Optional Alias1 Alias2...) , Image Link (, Optional Message) ;
static_image_command! {
    upload_log log, "https://cdn.discordapp.com/attachments/531598137790562305/575381000398569493/unknown.png", "Please upload your log:";
    select_java sjava, "https://cdn.discordapp.com/attachments/531598137790562305/575378380573114378/unknown.png", "Please select your Java version in the MultiMC settings:";
    select_memory smemory sram, "https://cdn.discordapp.com/attachments/531598137790562305/575376840173027330/unknown.png", "Please set your instance memory allocation:";
}
