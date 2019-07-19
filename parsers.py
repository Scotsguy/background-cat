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
            "Your MultiMC installation is in Program Files, where MultiMC doesn't have permission to write.\nMove it somewhere else, like your Desktop",
        )


check_java_version_regex = re.compile(r"Java is version (6|7|9|10|11|12)+\..+,")


def check_java_version(log):
    match = check_java_version_regex.search(log)
    if match:
        return (
            config.Severity.SEVERE,
            f"You're using Java {match.group(1)}, which is not supported by Minecraft. You should install Java 8 from [this link]({config.JAVA_LINK})",
        )


check_java_arch_regex = re.compile(
    r"Java is version .+, using 32-bit architecture\.\n+Your Java architecture is not matching your system architecture\."
)


def check_java_arch(log):
    match = check_java_arch_regex.search(log)
    if match:
        return (
            config.Severity.IMPORTANT,
            f"You're using 32-bit Java. You should install 64-bit Java from [this link]({config.JAVA_LINK})",
        )


check_ram_amount_regex = re.compile(r"(?<=-Xmx)(.+(?=[m]))")


def check_ram_amount(log):
    match = check_ram_amount_regex.search(log)
    if match:
        ram_amount = int(match.group(0))
        if ram_amount < 2000:
            return (
                config.Severity.IMPORTANT,
                f"You have only allocated {round(ram_amount/1000,1)}GB of RAM to Minecraft. This is too little, and you should raise it to at least 2GB",
            )
        elif ram_amount > 10000:
            return (
                config.Severity.WARNING,
                f"You have allocated {round(ram_amount/1000,1)}GB of RAM to Minecraft. [This is too much, and will cause lagspikes](https://vazkii.net/#blog/ram-explanation)",
            )


__all__ = [
    check_multimc_in_program_files,
    check_java_version,
    check_java_arch,
    check_ram_amount,
]
