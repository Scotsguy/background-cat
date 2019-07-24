import re

import config

check_multimc_in_program_files_regex = re.compile(
    r"Minecraft folder is:\nC:\/Program Files"
)


def check_multimc_in_program_files(log):
    match = check_multimc_in_program_files_regex.search(log)
    if match:
        return (
            config.Severity.SEVERE,
            "Your MultiMC installation is in Program Files, where MultiMC doesn't have permission to write.\nMove it somewhere else, like your Desktop.",
        )


check_server_java_regex = re.compile(r"OpenJDK .{2}-Bit Server VM warning")


def check_server_java(log):
    match = check_server_java_regex.search(log)
    if match:
        return (
            config.Severity.SEVERE,
            "You're using the server version of Java. You should install the desktop version from [this link]({config.JAVA_LINK}).",
        )


check_java_version_regex = re.compile(r"Java is version (6|7|9|10|11|12)+\..+,")


def check_java_version(log):
    match = check_java_version_regex.search(log)
    if match:
        return (
            config.Severity.SEVERE,
            f"You're using Java {match.group(1)}. Versions other than Java 8 are not designed to be used with Minecraft and may cause issues. You should install Java 8 from [this link]({config.JAVA_LINK}).",
        )


check_java_arch_regex = re.compile(
    r"Java is version .+, using 32-bit architecture\.\n+Your Java architecture is not matching your system architecture\."
)


def check_java_arch(log):
    match = check_java_arch_regex.search(log)
    if match:
        return (
            config.Severity.IMPORTANT,
            f"You're using 32-bit Java. You should install 64-bit Java from [this link]({config.JAVA_LINK}).",
        )


check_ram_amount_regex = re.compile(r"-Xmx(\d+)m[,\]]")


def check_ram_amount(log):
    match = check_ram_amount_regex.search(log)
    if match:
        ram_amount = int(match.group(1))
        if ram_amount > 10000:
            return (
                config.Severity.WARNING,
                f"You have allocated {round(ram_amount/1000,1)}GB of RAM to Minecraft. [This is too much and can cause lagspikes.](https://vazkii.net/#blog/ram-explanation)",
            )


check_class_not_found_regex = re.compile(
    r"Caused by: java\.lang\.ClassNotFoundException: (.+)(?![\s\S]+^Caused by:)",
    flags=re.MULTILINE,
)


def check_class_not_found(log):
    match = check_class_not_found_regex.search(log)
    if match:
        return (
            config.Severity.IMPORTANT,
            f"The following class was not found: {match.group(1)}. This is likely caused by a missing dependency.",
        )


__all__ = [
    check_multimc_in_program_files,
    check_java_version,
    check_class_not_found,
    check_java_arch,
    check_ram_amount,
    check_server_java,
]
