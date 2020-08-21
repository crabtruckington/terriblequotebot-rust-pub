# TerribleQuoteBot

This is a Quote Bot for Discord, written in Rust. 

To run it, you will need:

1) A machine running some version of linux. I used Debian to compile, and run the bot on a Raspberry Pi 4.
2) A Postgresql server. It can be local, or remote.
3) A copy of the Rust development environment, to build the source

Once the bot is running, it will respond to the following commands:
- `!quotehelp/!helpquote`, providing a list of standard commands to the user
- `!setactivity`, changing the bots current activity (This cycles every 30 minutes)
- `!quote {optional args}`, returns a quote. If a number is provided, returns that specific quote, or an error if outside the range of quotes. If no argument is provided, returns a random quote
- `!findquote/!quotesearch {args}`, searches the database for quotes containing the text provided
- `!addquote {args}`, adds a quote to the database
- `!delquote {args}`, deletes the specified quote
- `!ping {optional args}`, responds with `pong! {args}`. Useful to see if the bot is alive, or just to have a bot say things, which is always fun for users.

Note: `!setactivity` and `!delquote` require a role named `BotGod` to be present on the server, and the user who called the bot to have that role. Otherwise, they return an error and log the failed request, including the username and the arguments. You can edit the source of `commandparser.rs` to change the role it looks for if you want to use a role with another name, or remove the check all together if you trust your users (you shouldnt).

# Installation Steps:

Follow these steps to get the bot running. Creating a bot on the Discord site and Postgres installation are glossed over as its assumed you know how to do this. If you dont, there are many easy tutorials available for both of these topics.

1.  Create a bot on the Discord API developers site, and copy your `token` (NOT your ClientID, your secret `token`). Invite the bot to your server.
1.  Set up a Postgres server, create a database (the default database name is `quotes`, but you can choose to name it anything) and then create a table in that database also called `quotes` with the following schema: `CREATE TABLE quotes ( id INT GENERATED ALWAYS AS IDENTITY, quote VARCHAR NOT NULL )`. It is highly recommended to create a new user in Postgres at this point, with full access to the database, as the Bot will need to create and drop tables. You can use the default `postgres` user, but you shouldnt as it has root access.
    1.  At this point, you can optionally load a list of quotes into the database. I believe you will need at least 1 quote for most of the functionality except `!addquote` and `!ping` to not panic, so go ahead and `INSERT INTO quotes (quote) VALUES ('test');` if you dont have a list of quotes ready right now.
1.  Clone or download the source of this repository to your local machine
1.  Navigate to the `./src` directory of the repository and rename `configs.rs.example` to `configs.rs`
1.  Open `configs.rs` and enter the values for your bot. You will need your Discord bot `token`, and the connection string values for your Postgres server (If your database is not called `quotes`, dont forget to add the proper database to the connection string!). Optionally, you can change the prefix the bot responds to. By default this is `!`. If you do change this, you will need to modify `commandparser.rs` and change the `match` statement to include your new prefix. 
1.  Navigate up one directory, to the folder that contains `Cargo.toml`, and build the source using `cargo build --release`
1.  Once it is finished compiling, navigate to the new created `/target/release` and run `./TerribleQuoteBot` to start the bot. If you are unfamiliar with Rust, the `TerribleQuoteBot` binary is the only file you need to run the program, so you can copy this file elsewhere and delete everything else in `./target/release`. The Rust build process creates a lot of artifacts to speed up future compilations, so this folder can grow quite large.
1.  Go to your Discord server and try calling the bot using `!ping`. If the bot is running correctly, it should respond. If not, check the console to review the error.

Optionally, you can configure the bot as a service, so it starts whenever your computer starts. There are many tutorials that cover how to do this. Just keep in mind that Rust compiles to a binary, so there is no need to have your service do anything except start the program, and optionally redirect the `stdout` and `stderror` to a log file. The bot is quite chatty, so some sort of log rotation would be recommended.

This is an example of my `terriblequotebot.service` config file for systemd:

```
Description=A_Terrible_Quote_Bot_In_Rust

Wants=network.target
After=syslog.target network-online.target postgresql.service

[Service]
Type=simple
ExecStart=/release/TerribleQuoteBot
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=terriblequotebot
Restart=on-failure
RestartSec=10
KillMode=process

[Install]
WantedBy=multi-user.target
```

If you have `rsyslog`, you can configure that to rewrite the logs to a separate file, otherwise they will end up in the default syslog.

# NOTES:

- This bot uses a forked version of Serenity 0.8.7 (https://github.com/serenity-rs/serenity) to implement some undocumented activities, and thus builds it from local source using the `patch` option of Cargo. If you want, you can remove this dependency by editing the activities array in `commandparser.rs` to remove reference to the `watching` activity. Then modify `cargo.toml` as appropriate. I do not see any good reason to do this though, and of course this is completely unsupported. If you are worried about this, go ahead and diff it against serenitys official 0.8.7, I only implemented the activities and added build flags to ignore the compilation warnings (so Cargo doesnt dump out your secret tokens all over the console during build).
- This /might/ compile properly on windows and mac, I havent tried. I dont see any reason why it wouldnt, but obviously this is unsupported.
- This was mostly a fun project, to see if I could clone a bot I already wrote using Rust. You should probably not use this as an example of how to do anything, either in Rust itself or the libraries I used.
