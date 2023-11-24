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

## Day 4: 2023-11-07

Extended money precision to 1000th of a cent, and added tests for it.
Tweaked total stats UI component.
Added price UI to cloud services.
Implemented compact presentations for op counts (e.g. "10M base ops").
Tweak styles some more.

## Day 5: 2023-11-09

Added UI for computational nodes in a single rack.
Added preliminary event reactor construct
(PRNG event sampler).

## Day 6: 2023-11-11

Reworked timestamps and logic for working with game time
(there are now approximately N time units per millisecond).
Added player action data structure.
Initiated work on active game event generation engine.
Added first revision of user action message.

## Day 7: 2023-11-12

Added modal UI component.
Added state, engine, and watch to playground,
added preliminary logic to apply player actions
and apply card effects.
Refactored business and total stats component.

## Day 8: 2023-11-14

Continued extending card effects and conditions.
Added state serialization.
Tweaked card functioning stuff, including the use of strings to identify cards.
Added more properties to cloud services.
Used global card registry to playground,
making it closer to how the main game will work.

## Day 9: 2023-11-15

Added audio, with 2 click sounds.
Adjusted asset copying on Trunk so that it copies the whole assets dir.
Added a bit of logic for changing service price.

## Day 10: 2023-11-16

Worked more on the logic for changing service price.

## Day 11: 2023-11-18

Finished implementing baseline cloud request handling,
so that requests arrive based on demand & price
and that processing them results in goodies.
Transferred more world state information to playground.
Implemented node hardware updates.

## Day 12: 2023-11-19

Implemented electricity consumption & bills (powersaveing not done yet).
Readjusted demand and processing time calculations.
Added periodic game saving and periodic base demand increase over time.
Implemented untested logic for DoS attacks caused by high demand.

## Day 13: 2023-11-20

Implemented more complex memory reserve calculations.
Implemented caching: more memory reserved per level, but higher cache hit rate.
Added more card effects, some of them currently stubs.
Implemented ending user specifications with trial periods.
Added more cards to the full list.
Tweaked Cargo manifest to optimize some dependencies in dev mode.

## Day 14: 2023-11-21

Implemented root application & main menu.
Game now starts in main menu with "New Game" and "Continue" options.
Added id to CloudUserSpec,
so that now we query user specifications by this ID
instead of by vector index,
which can change.
Bootstraped events for freshly created specs,
otherwise they wouldn't do anything.
Changed electricity cost to cost levels,
and added card effect to change this level.

## Day 15: 2023-11-22

Implemented entitlements:
get money from ops even if you're the one clicking!
Added many more card specifications.
Fixed some other issues found in
request generation and game saving/loading.
Added CI workflow to deploy the game automatically to GitHub Pages.

## Day 16: 2023-11-23

Fixed many serious bugs with cards and event queue.
Improved engine resilience to missing user specs.
Extended cards.
Made some cloud service info transients so that they don't serialize.
Improved styles.
Worked on main menu a bit more,
including warning the player if the browser can't save the game.
Changed costs substantially.
Adjusted demand calculation and card conditions.

## Day 17: 2023-11-24

Fixed serious bugs, one in event bootstrapping and the other one in checking whether a card was used.
Adjusted cards and hardware costs & values.
Tweaked demand formula again.
Implemented card effects for unlocking multiple nodes, racks, etc.
Tweaked price changing UI, but more changes may be needed 
Added more cards and card conditions.
