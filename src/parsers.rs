#![deny(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

pub(crate) type Check = fn(&str) -> Option<(&str, String)>;

pub(crate) const PARSERS: [Check; 14] = [
    multimc_in_program_files,
    macos_too_new_java,
    multimc_in_onedrive_managed_folder,
    //major_java_version,
    forge_too_new_java,
    one_seventeen_java_too_new,
    m1_failed_to_find_service_port,
    pixel_format_not_accelerated_win10,
    intel_graphics_icd_dll,
    id_range_exceeded,
    out_of_memory_error,
    shadermod_optifine_conflict,
    fabric_api_missing,
    java_architecture,
    old_multimc_version,
];

fn multimc_in_program_files(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Minecraft folder is:\nC:/Program Files";
    if log.contains(TRIGGER) {
        Some(("‼", "Your MultiMC installation is in Program Files, where MultiMC doesn't have permission to write.\n\
        You should move it somewhere else, like your Desktop.".to_string()))
    } else {
        None
    }
}

fn macos_too_new_java(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = r#"Terminating app due to uncaught exception 'NSInternalInconsistencyException', reason: 'NSWindow drag regions should only be invalidated on the Main Thread!'"#;
    if log.contains(TRIGGER) {
        Some(("‼", "You are using too new a Java version. Please follow the steps on this wiki page to install 8u241: https://github.com/MultiMC/MultiMC5/wiki/Java-on-macOS".to_string()))
    } else {
        None
    }
}

fn id_range_exceeded(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str =
        "java.lang.RuntimeException: Invalid id 4096 - maximum id range exceeded.";
    if log.contains(TRIGGER) {
        Some(("‼", "You've exceeded the hardcoded ID Limit. Remove some mods, or install [JustEnoughIDs](https://www.curseforge.com/minecraft/mc-mods/jeid)".to_string()))
    } else {
        None
    }
}

fn out_of_memory_error(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "java.lang.OutOfMemoryError";
    if log.contains(TRIGGER) {
        Some(("‼", "You've run out of memory. You should allocate more, although the exact value depends on how many mods you have installed. \
        [Click this link for a guide.](https://cdn.discordapp.com/attachments/531598137790562305/575376840173027330/unknown.png)".to_string()))
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

fn fabric_api_missing(log: &str) -> Option<(&str, String)> {
    const EXCEPTION: &str =
        "net.fabricmc.loader.discovery.ModResolutionException: Could not find required mod:";
    const FABRIC: &str = "requires {fabric @";

    if log.contains(EXCEPTION) && log.contains(FABRIC) {
        Some((
            "‼",
            "You are missing Fabric API, which is required by a mod. \
        [Download the needed version here](https://www.curseforge.com/minecraft/mc-mods/fabric-api)"
                .to_string(),
        ))
    } else {
        None
    }
}

fn multimc_in_onedrive_managed_folder(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Minecraft folder is:\nC:/.+/.+/OneDrive").unwrap();
    }
    if RE.is_match(log) {
        Some(("❗", "MultiMC is located in a folder managed by OneDrive. OneDrive messes with Minecraft folders while the game is running, \
        and this often leads to crashes.\nYou should move the MultiMC folder to a different folder.".to_string()))
    } else {
        None
    }
}
/*
fn major_java_version(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Java is version (1.)??(?P<ver>[6-9]|[1-9][0-9])+(\..+)??,").unwrap();
    }
    match RE.captures(log) {
        Some(capture) if capture.name("ver")?.as_str() == "8" => None,
        Some(capture) => Some((
            "❗",
            format!(
                "You're using Java {}. Versions other than Java 8 are not designed to be used with Minecraft and may cause issues. \
                [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)",
                capture.name("ver")?.as_str()
            ),
        )),
        _ => None,
    }
}
*/

fn forge_too_new_java(log: &str) -> Option<(&str, String)> {
    const URLCLASSLOADER_CAST: &str = "java.lang.ClassCastException: class jdk.internal.loader.ClassLoaders$AppClassLoader cannot be cast to class java.net.URLClassLoader";
    if log.contains(URLCLASSLOADER_CAST) {
        Some(("‼", "The version of Minecraft you are playing does not support using modern versions of Java. \
        [Please use Java 8, click here for help.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)".to_string()))
    } else {
        None
    }
}

fn one_seventeen_java_too_new(log: &str) -> Option<(&str, String)> {
    const UNSUPPORTED_CLASS_VERSION_ERROR: &str =
        "java.lang.UnsupportedClassVersionError: net/minecraft/client/main/Main";
    const FABRIC_JAVA_VERSION_ERROR: &str = "fabric requires {java @ [>=16]}";
    if log.contains(UNSUPPORTED_CLASS_VERSION_ERROR) || log.contains(FABRIC_JAVA_VERSION_ERROR) {
        Some(("‼", "You are playing a version of Minecraft that requires Java 16, but are using an older Java version. \n\
        Please install Java 16 you can find downloads [here](https://www.azul.com/downloads/?version=java-16-sts&architecture=x86-64-bit&package=jre#download-openjdk))\n\
        On Windows: Download the .msi file. After installation you may have to update MultiMC to detect the new Java version, to do so open the settings and \
        change Update Channel to 'Development', then update MultiMC.\n\
        Open the MultiMC Java settings and make sure Java 8 is still selected as default for more help with that run `-sjava`.\
        Then edit your 1.17 instance settings, open the Java tab, check 'Java Installation', click 'Auto-detect..' and select Java 16.".to_string()))
    } else {
        None
    }
}

fn m1_failed_to_find_service_port(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "java.lang.IllegalStateException: GLFW error before init: [0x10008]Cocoa: Failed to find service port for display";
    if log.contains(TRIGGER) {
        Some(("‼", "You seem to be using an Apple M1 Mac with an incompatible version of Forge. Add the following to your launch arguments as a workaround: `-Dfml.earlyprogresswindow=false`".to_string()))
    } else {
        None
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

fn intel_graphics_icd_dll(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"C  \[(ig[0-9]+icd[0-9]+\.dll)\+(0x[0-9a-f]+)\]").unwrap();
    }
    if RE.is_match(log) {
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
