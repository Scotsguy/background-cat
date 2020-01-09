#![deny(dead_code)]

use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

pub type Check = fn(&str) -> Option<(&str, String)>;

pub static PARSERS: [Check; 8] = [
    multimc_in_program_files,
    server_java,
    buildsystem_forge,
    java_version,
    id_range_exceeded,
    out_of_memory_error,
    java_architecture,
    ram_amount,
];

fn multimc_in_program_files(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Minecraft folder is:\nC:/Program Files";
    if log.contains(TRIGGER) {
        Some(("‼", "Your MultiMC installation is in Program Files, where MultiMC doesn't have permission to write.\nMove it somewhere else, like your Desktop.".to_string()))
    } else {
        None
    }
}

fn server_java(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "-Bit Server VM warning";
    if log.contains(TRIGGER) {
        Some(("‼", "You're using the server version of Java. [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)".to_string()))
    } else {
        None
    }
}

fn buildsystem_forge(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"net\.minecraftforge/(?P<major>(2[5-9]|30))\.[0-9]+\.[0-9]+\.json")
                .unwrap();
    }
    if let Some(capture) = RE.captures(log) {
        let mc_version = match capture.name("major")?.as_str() {
            "25" => "1.13.2",
            "26" => "1.14.2",
            "27" => "1.14.3",
            "28" => "1.14.4",
            "29" => "1.15",
            "30" => "1.15.1",
            _ => "<unknown version>",
        };

        Some(("‼", format!(
             "You're trying to use Forge for Minecraft version {}. This is not supported by MultiMC. For more information, please see [this link.](https://multimc.org/posts/forge-114.html)",
             mc_version)
            ))
    } else {
        None
    }
}

fn id_range_exceeded(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str =
        "java.lang.RuntimeException: Invalid id 4096 - maximum id range exceeded.";
    if log.contains(TRIGGER) {
        Some(("‼", "You've exceeded the hardcoded ID Limit. Remove some mods, or install [this one](https://www.curseforge.com/minecraft/mc-mods/notenoughids)".to_string()))
    } else {
        None
    }
}
fn out_of_memory_error(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "java.lang.OutOfMemoryError";
    if log.contains(TRIGGER) {
        Some(("‼", "You've run out of memory. You should allocate more, although the exact value depends on how many mods you have installed.".to_string()))
    } else {
        None
    }
}

fn java_version(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Java is version (1.)??(?P<ver>6|7|9|10|11|12)+\..+,").unwrap();
    }
    if let Some(capture) = RE.captures(log) {
        Some(("❗", format!("You're using Java {}. Versions other than Java 8 are not designed to be used with Minecraft and may cause issues. [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)",
            capture.name("ver")?.as_str())))
    } else {
        None
    }
}

fn java_architecture(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Your Java architecture is not matching your system architecture.";
    if log.contains(TRIGGER) {
        Some(("❗", "You're using 32-bit Java. You should install 64-bit Java from [this link](https://java.com/en/download/manual.jsp).".to_string()))
    } else {
        None
    }
}

fn ram_amount(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-Xmx(?P<amount>\d+)m[,\]]").unwrap();
    }
    if let Some(capture) = RE.captures(log) {
        let amount = capture.name("amount")?.as_str().parse::<f32>();
        let amount = match amount {
            Ok(o) => o,
            Err(why) => {
                warn!("Couldn't parse RAM amount: {:?}", why);
                return None;
            }
        };
        let amount = amount / 1000.0; // Megabytes => Gigabytes

        if amount > 10.0 {
            return Some(("⚠", format!("You have allocated {}GB of RAM to Minecraft. [This is too much and can cause lagspikes.](https://vazkii.net/#blog/ram-explanation)", amount )));
        };
    }
    None
}
