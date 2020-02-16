#![deny(dead_code)]

use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

pub(crate) type Check = fn(&str) -> Option<(&str, String)>;

pub(crate) const PARSERS: [Check; 12] = [
    multimc_in_program_files,
    server_java,
    buildsystem_forge,
    multimc_in_onedrive_managed_folder,
    major_java_version,
    pixel_format_not_accelerated_win10,
    id_range_exceeded,
    out_of_memory_error,
    shadermod_optifine_conflict,
    java_architecture,
    old_multimc_version,
    ram_amount,
];

fn multimc_in_program_files(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Minecraft folder is:\nC:/Program Files";
    if log.contains(TRIGGER) {
        Some(("‼", "Your MultiMC installation is in Program Files, where MultiMC doesn't have permission to write.\nYou should move it somewhere else, like your Desktop.".to_string()))
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
            Regex::new(r"net\.minecraftforge/(?P<major>(2[5-9]|3[0-1]))\.[0-9]+\.[0-9]+\.json")
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
            "31" => "1.15.2",
            _ => "<unknown version>",
            // When adding new versions, change the regex too
        };

        Some((
            "‼",
            format!(
                "You're trying to use Forge for Minecraft version {}. \
              This is not supported by MultiMC. For more information, please see \
              [this link.](https://multimc.org/posts/forge-114.html)",
                mc_version
            ),
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

fn shadermod_optifine_conflict(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "java.lang.RuntimeException: Shaders Mod detected. Please remove it, OptiFine has built-in support for shaders.";
    if log.contains(TRIGGER) {
        Some(("‼", "You've installed Shaders Mod alongside OptiFine. OptiFine has built-in shader support, so you should remove Shaders Mod".to_string()))
    } else {
        None
    }
}

fn multimc_in_onedrive_managed_folder(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Minecraft folder is:\nC:/.+/.+/OneDrive").unwrap();
    }
    if RE.is_match(log) {
        Some(("❗", "MultiMC is located in a folder managed by OneDrive. OneDrive messes with Minecraft folders while the game is running, and this often leads to crashes.\nYou should move the MultiMC folder to a different folder.".to_string()))
    } else {
        None
    }
}

fn major_java_version(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Java is version (1.)??(?P<ver>[1-9][0-9]?)+\..+,").unwrap();
    }
    match RE.captures(log) {
        Some(capture) if capture.name("ver")?.as_str() == "8" => None,
        Some(capture) => Some((
            "❗",
            format!(
                "You're using Java {}. Versions other than Java 8 are not designed to be used with Minecraft and may cause issues. [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)",
                capture.name("ver")?.as_str()
            ),
        )),
        _ => None,
    }
}

fn pixel_format_not_accelerated_win10(log: &str) -> Option<(&str, String)> {
    const LWJGL_EXCEPTION: &str = "org.lwjgl.LWJGLException: Pixel format not accelerated";
    const WIN10: &str = "Operating System: Windows 10";
    if log.contains(LWJGL_EXCEPTION) && log.contains(WIN10) {
        Some(("❗", "You seem to be using an Intel GPU that is not supported on Windows 10. \
         You will need to install an older version of Java, [see here for help](https://github.com/MultiMC/MultiMC5/wiki/Unsupported-Intel-GPUs)".to_string()))
    } else {
        None
    }
}

fn java_architecture(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Your Java architecture is not matching your system architecture.";
    if log.contains(TRIGGER) {
        Some(("❗", "You're using 32-bit Java. [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)".to_string()))
    } else {
        None
    }
}

fn old_multimc_version(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"MultiMC version: (?P<major_ver>0\.[0-9]+\.[0-9]+-(?P<build>[0-9]+))\n")
                .unwrap();
    }
    if let Some(capture) = RE.captures(log) {
        match capture.name("build")?.as_str().parse::<u32>() {
            Ok(o) => {
                if o < 900 {
                    Some((
                        "❗",
                        format!(
                            "You seem to be using an old build of MultiMC ({}). \
                            Please update to a more recent version.",
                            capture.name("major_ver")?.as_str()
                        ),
                    ))
                } else {
                    None
                }
            }
            Err(_) => Some((
                "❗",
                format!(
                    "You seem to be using an unofficial version of MultiMC ({}). \
                    Please only use MultiMC downloaded from [multimc.org](https://multimc.org/#Download).",
                    capture.name("major_ver")?.as_str()
                ),
            )),
        }
    } else {
        None
    }
}

fn ram_amount(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-Xmx(?P<amount>[0-9]+)m[,\]]").unwrap();
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
            return Some((
                "⚠",
                format!(
                    "You have allocated {}GB of RAM to Minecraft. [This is too much and can cause lagspikes.](https://vazkii.net/#blog/ram-explanation)",
                    amount
                ),
            ));
        };
    }
    None
}
