import enum

TOKEN = ""
PRIVILEDGED_USERS = [185461862878543872, 238711994847461376]
PRIVILEDGED_ROLES = [
    311142723518464000,  # Regular
    134403532873793536,  # Moderator
    269384758914449409,  # Overlord
]  # These roles can delete messages the bot sends by reacting with :no_entry_sign:
# Important: Compared with the highest role of a member


JAVA_LINK = "https://java.com/en/download/manual.jsp"


class Severity(enum.Enum):
    SEVERE = "\N{DOUBLE EXCLAMATION MARK}"
    IMPORTANT = "\N{HEAVY EXCLAMATION MARK SYMBOL}"
    WARNING = "\N{WARNING SIGN}"
