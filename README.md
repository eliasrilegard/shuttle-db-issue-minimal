# shuttle-db-issue-minimal
A minimal setup that causes the Discord bot to stop receiving and/or processing interaction events

## Usage
1. Clone the repository
2. Update `DISCORD_TOKEN` in `Secrets.toml`
2. Have docker running to contain the database
3. Start up: `$ cargo shuttle run`

In [`src/database.rs`](src/database.rs), depending on whether the match block with `get_reminders` (line 29) is
commented out or not, the bot will behave differently. If the block is commented out, the bot will behave like normal.
If the block is included (and by extension the database is queried), the bot will seemingly completely freeze and not
respond to any command interactions once it's queried the database for the first time.
