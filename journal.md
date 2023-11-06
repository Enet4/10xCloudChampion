# GitHub GameOff 2023 journal

## Day 1: 2023-11-04

Created a new repository.
Came up with the name of the game.
Created the first iteration of some of the UI components (including styles)
for cards, panels, load bars, and business stats.
Added a different font, _PT Sans_.
Created also base data types for `Money` (with fixed precision)
and `Cost` (of using a card, for example).
Wrote down some ideas for cards.

## Day 2: 2023-11-05

Added data type `Ops`, `Memory`.
Added numerical indicators for CPU and memory load.
Added more card logic (`CardSpec`, `CardCondition`, `CardEffect`),
including definining what will become the full list of cards.
Started working on request events and request event queue.
Copied and adapted `GameWatch` from 10x Sprint Master.
Added pop-up thingies whenever the user presses the cloud service Op button.
Started work on Cloud users, their specification and live details.

## Day 3: 2023-11-06

Set up a development container so that I can work on the game from GitHub Codespaces.
Edited the readme and tweaked Cargo.toml.
Fixed tests for `Separating` display component.
Tweaked styles a bit.
