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
            f"You're using Java {match.group(1)}, which is not supported. You should install Java 8 from [this link]({config.JAVA_LINK})",
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


__all__ = [check_multimc_in_program_files, check_java_version, check_java_arch]
