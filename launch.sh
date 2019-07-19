#!/usr/bin/env bash
while true ; do
    poetry run python main.py
    case $? in
        7)
        echo 'Updating from git'
        git pull
        ;&
        6)
        echo 'Restarting'
        continue
        ;;
        *)
        break
        ;;
    esac
done