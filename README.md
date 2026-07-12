# Mahjong Elo Calculator by Tsehy

This CLI tool is for tracking our regular mahjong game nights based on an ELO system.
To calculate the ranking of a four player game the software generates and evaluates six simultaneous 1v1 games.

## Features/Commands

- Init Season
- Add game to a season
- Display quick information
- Export summary to JSON (For [carl.gg](https://carl.gg) discord embed)

## ToDo list

- [ ] Add player command to not rely only on `--force` when adding a game
- [ ] Remove game command to be able to correct mistakes