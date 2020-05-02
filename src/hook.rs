use log::error;
use serenity::{
    framework::standard::{macros::hook, CommandError},
    model::channel::Message,
    prelude::*,
};

#[hook]
pub(crate) async fn after_hook(
    _: &Context,
    _: &Message,
    cmd_name: &str,
    result: Result<(), CommandError>,
) {
    if let Err(why) = result {
        error!("Command `{}` returned with an error: {:?}", cmd_name, why);
    }
}
