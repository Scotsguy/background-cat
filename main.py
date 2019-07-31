import asyncio
import logging
import re
import datetime

import aiohttp
import discord

import config
import parsers
import secret

logger = logging.getLogger("discord")
logger.setLevel(logging.DEBUG)
handler = logging.FileHandler(filename="background-cat.log", encoding="utf-8", mode="w")
handler.setFormatter(
    logging.Formatter("%(asctime)s:%(levelname)s:%(name)s: %(message)s")
)
logger.addHandler(handler)

client = discord.Client(activity=discord.Game("DM me!"), guild_subscriptions=False)

# Expand to more providers, perhaps?
pastee_regex = re.compile(r"https:\/{2}paste.ee\/p\/[^\s/]+")


async def handle_common_mistakes(log):

    info_text = [func(log) for func in parsers.__all__]
    info_text = list(filter(None.__ne__, info_text))
    if not info_text:
        return

    embed = discord.Embed(
        title="Automated Response (Warning: Experimental)",
        colour=discord.Colour.dark_teal(),
        timestamp=datetime.datetime.utcnow(),
    ).set_footer(
        text="This might not solve your problems, but it could be worth a try",
        icon_url="https://cdn.discordapp.com/emojis/280120125284417536.png?v=1",
    )

    for field in info_text:
        embed.add_field(name=field[0].value, value=field[1], inline=True)

    return embed


async def handle_self_delete(message):
    def deletion_check(reaction, user):
        user_allowed = (
            user.id in config.PRIVILEDGED_USERS
            or user.top_role.id in config.PRIVILEDGED_ROLES
        )
        return user_allowed and str(reaction.emoji) == "\N{NO ENTRY SIGN}"

    await message.add_reaction("\N{NO ENTRY SIGN}")
    try:
        await client.wait_for(
            "reaction_add", timeout=config.DELETE_TIMEOUT, check=deletion_check
        )
    except asyncio.TimeoutError:
        await message.remove_reaction("\N{NO ENTRY SIGN}", message.guild.me)
    else:
        await message.delete()


@client.event
async def on_ready():
    logging.getLogger("discord").info(f"Logged in as: {client.user}")
    if not hasattr(client, "httpsession"):
        client.httpsession = aiohttp.ClientSession()


@client.event
async def on_message(message):
    if message.author.bot:
        return

    if message.guild is None or message.guild.me.mentioned_in(message):
        if message.author.id in config.OWNERS and message.content.startswith("stop"):
            await message.channel.send("Stopping...")
            await client.logout()

        info_embed = discord.Embed(
            title="<:backgroundcat:280120125284417536>A bot to parse logfiles on the MultiMC discord<:backgroundcat:280120125284417536>",
            description=f"""
            Developed by {str(client.get_user(185461862878543872))}.
            To start, just post a https://paste.ee link in the Discord.
            
            [Source Code available under AGPLv3](https://gitlab.com/Scotsguy/background-cat)
            """,
            colour=discord.Colour.teal(),
            timestamp=datetime.datetime.utcnow(),
        )
        await message.author.send(embed=info_embed)

    link = pastee_regex.search(message.content)
    if not link:
        return
    link = link.group(0).replace("/p/", "/r/", 1)  # get raw paste
    async with client.httpsession.get(link) as resp:
        log = await resp.text()

    mistakes_embed = await handle_common_mistakes(log)
    if mistakes_embed:
        await handle_self_delete(await message.channel.send(embed=mistakes_embed))


client.run(secret.TOKEN)
