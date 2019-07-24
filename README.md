# Background Cat
## A Discord bot to parse MultiMC Logs and warn users about common errors
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/ambv/black)

I made this bot because it was brought to my attention that other discords also had similar bots. Dissatisfied with the other ones (and because I can't write what any of them are written in), I made yet another one.

[![xkcd "Standards" comic](https://imgs.xkcd.com/comics/standards.png)](https://xkcd.com/927/)


## Organization

If you want to add a new warning to the bot, make a function in `parsers.py` that takes in the full log and returns a tuple of a severity (see `config.example.py`) and a string. Don't forget to add it to `__all__`, as this is what actually gets iterated over.