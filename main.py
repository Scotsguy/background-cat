import asyncio
import logging

import aiohttp
import discord

import config
import parsers
import re

logging.basicConfig(level=logging.INFO)

client = discord.Client()

# Expand to more providers, perhaps?
pastee_regex = re.compile(r"https:\/{2}paste.ee\/p\/[^\s/]+")


@client.event
async def on_ready():
    print(f"Logged in as: {client.user}")
    if not hasattr(client, "httpsession"):
        client.httpsession = aiohttp.ClientSession()


@client.event
async def on_message(message):
    if message.author.bot:
        return
    if message.guild is None:
        return

    if message.guild.me.mentioned_in(message):
        info_embed = discord.Embed(
            title="<:backgroundcat:280120125284417536>A bot to parse logfiles on the MultiMC discord<:backgroundcat:280120125284417536>",
            description="Developed by AppleTheGolden#7645.\n\n[Source Code available under AGPLv3](https://github.com/Scotsguy/parserbot)",
            colour=discord.Colour.teal(),
        )
        await message.author.send(embed=info_embed)

    link = pastee_regex.search(message.content)
    if not link:
        return
    link = link.group(0).replace("/p/", "/r/", 1)  # get raw paste
    async with client.httpsession.get(link) as resp:
        log = await resp.text()

    info_text = [func(log) for func in parsers.__all__]
    info_text = list(filter(None.__ne__, info_text))
    if not info_text:
        return

    embed = discord.Embed(
        title="Automated Response (Warning: Experimental)",
        colour=discord.Colour.dark_teal(),
    ).set_footer(text="This might not solve your problems, but it could be worth a try")

    for field in info_text:
        embed.add_field(name=field[0].value, value=field[1], inline=True)

    info_message = await message.channel.send(embed=embed)

    # Reaction to delete the message if it's not helpful (by priviledged roles only)
    # ==================================================================================
    def deletion_check(reaction, user):
        user_allowed = (
            user.id in config.PRIVILEDGED_USERS
            or user.top_role.id in config.PRIVILEDGED_ROLES
        )
        return user_allowed and str(reaction.emoji) == "\N{NO ENTRY SIGN}"

    await info_message.add_reaction("\N{NO ENTRY SIGN}")
    try:
        await client.wait_for("reaction_add", timeout=120.0, check=deletion_check)
    except asyncio.TimeoutError:
        await info_message.remove_reaction("\N{NO ENTRY SIGN}", message.guild.me)
    else:
        await info_message.delete()


client.run(config.TOKEN)
