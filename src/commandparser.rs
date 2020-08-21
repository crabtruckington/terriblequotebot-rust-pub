#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::*;
use rand::prelude::*;
use serenity::
{
    prelude::*,
    model::prelude::*,
    model::gateway::Activity,
    model::channel::Message,
};
#[path = "sqltools.rs"]
mod sqltools;
#[path = "configs.rs"]
mod configs;

// this value is used to prevent statuses from being repeated
// set it to an unreasonably high number so we always take the first status offered on startup
static mut lastActivityNumber: usize = 50;

pub fn parseCommand(command: String, args: String, context: Context, msg: Message)
{
    let isBotGod = isBotGod(msg.clone(), context.clone());

    match command.as_str()
    {
        "!quotehelp" | "!helpquote" =>
        {
            let sendDiscordMessage = || -> Result<(), serenity::Error>
            {
                msg.channel_id.say(&context.http, "The following commands are supported:")?;
                msg.channel_id.say(&context.http, "!quote: shows a random quote, optionally provide a number to show a specific quote")?;
                msg.channel_id.say(&context.http, "!addquote <text>: adds a quote to the database")?;
                msg.channel_id.say(&context.http, "!findquote <text> (alias: !quotesearch): search for a quote containing this text")?;
                msg.channel_id.say(&context.http, "!ping: pong!")?;
                Ok(())
            };
            if let Err(_err) = sendDiscordMessage()
            {
                println!("Error sending message: {}", _err);
            }
        }
        "!setactivity" =>
        {
            if (isBotGod)
            {
                setActivityType(context.clone());
                if let Err(err) = msg.channel_id.say(&context.http, "Activity changed!")
                {
                    println!("Error sending message {}", err);
                }
            }
            else
            {
                if let Err(err) = msg.channel_id.say(&context.http, "You do not have permissions to change the activity.")
                {
                    println!("Error sending message {}", err);
                }
            }
        }
        "!ping" =>
        {
            if let Err(why) = msg.channel_id.say(&context.http, ["pong! ", &args].concat())
            {
                println!("Error sending message: {}", why);
            }
        }
        "!quote" =>
        {
            if (args.trim().is_empty())
            {
                println!("Getting random quote...");
                let returnedQuote = sqltools::getRandomQuoteFromDatabase();
                if let Err(err) = msg.channel_id.say(&context.http, returnedQuote)
                {
                    println!("Error sending message {}", err);
                }
            }
            else
            {
                let mut returnedQuote: String = "".to_string();
                match args.parse::<i32>()
                {
                    Ok(quoteNumber) => returnedQuote = sqltools::getSpecificQuoteFromDatabase(&quoteNumber),
                    Err(_e) => returnedQuote = format!("You have not provided a valid selection! Valid selections are from 1 - {}!",
                    sqltools::getTotalQuoteValueFromDatabase().to_string()),
                }
                if let Err(err) = msg.channel_id.say(&context.http, returnedQuote)
                {
                    println!("Error sending message {}", err);
                }
            }
        }
        "!findquote" | "!quotesearch" =>
        {
            if (args.trim().is_empty())
            {
                println!("No search term provided, sending error message...");
                if let Err(err) = msg.channel_id.say(&context.http, "You didn't provide a search term! Please try again.")
                {
                    println!("Error sending message {}", err);
                }
            }
            else
            {
                println!("Starting search for quotes with text '{}'", args.to_string());
                let mut message: String = "".to_string();
                let returnedQuotes: String = sqltools::findQuotesByText(args.trim().to_string());
                if (returnedQuotes == "".to_string())
                {
                    message = format!("No quotes were found containing the text '{}'", args);
                    if let Err(err) = msg.channel_id.say(&context.http, message)
                    {
                        println!("Error sending message {}", err);
                    }   
                }
                else
                {
                    message = format!("The following quotes were found: {}", returnedQuotes);
                    message.truncate(message.len() -2);
                    if let Err(err) = msg.channel_id.say(&context.http, message)
                    {
                        println!("Error sending message {}", err);
                    }                    
                }
    
            }
        }
        "!addquote" =>
        {
            if (args.trim().is_empty())
            {
                println!("No quote text provided, sending error message...");
                if let Err(err) = msg.channel_id.say(&context.http, "You didn't provide a quote! Please provide some text to quote.")
                {
                    println!("Error sending message {}", err);
                }
            }
            else
            {
                println!("Adding quote to database with text '{}'", args.to_string());
                let newQuoteID: i32 = sqltools::addQuoteToDatabase(args);

                if let Err(err) = msg.channel_id.say(&context.http, format!("Quote {} added to the database!", newQuoteID))
                {
                    println!("Error sending message {}", err);
                }
            }
        }
        "!delquote" =>
        {
            if (isBotGod)
            {
                let mut returnedQuote: String = "".to_string();
                match args.parse::<i32>()
                {
                    Ok(quoteNumber) => returnedQuote = sqltools::deleteQuoteFromDatabase(&quoteNumber),
                    Err(_e) => returnedQuote = format!("Quote {} doesnt exist! Valid selections are from 1 - {}!", args.to_string(),
                    sqltools::getTotalQuoteValueFromDatabase().to_string()),
                }

                if let Err(err) = msg.channel_id.say(&context.http, returnedQuote)
                {
                    println!("Error sending message {}", err);
                }
            }
            else
            {
                let a: String = msg.author.name;
                println!("{} tried to delete a quote, using these arguments: {}", a, args);
                if let Err(err) = msg.channel_id.say(&context.http, "You are not allowed to delete quotes.")
                {
                    println!("Error sending message {}", err);
                }
            }
        
        }
        // "!getallquotes" =>
        // {
                //we dont need this anymore since we control the database directory. We can back it up instead
        // }

        // ignore commands with our prefix but without a valid command for this bot
        _ =>
        {
            println!("Command {} not for us, ignoring...", command);
        }
    }
}

pub fn setActivityType(context: Context)
{
    println!("Setting new activity...");
    let mut rng = thread_rng();
    let activityArray: [Activity;18] = [ Activity::watching("you sleep"),
                                         Activity::watching("from the bushes"),
                                         Activity::watching("you poop"),
                                         Activity::watching("every move you make"),
                                         Activity::watching("Perfect Strangers"),
                                         Activity::watching("machine elves"),
                                         Activity::watching("the pretty lights"),
                                         Activity::playing("with your emotions"),
                                         Activity::playing("with your wife"),
                                         Activity::playing("doctor"),
                                         Activity::playing("HoboGarbageSimulator"),
                                         Activity::listening("In Your Eyes"),
                                         Activity::listening("the voices"),
                                         Activity::listening("silence"),
                                         Activity::listening("your phone calls"),
                                         Activity::listening("talk radio"),
                                         Activity::listening("the transmissions"),
                                         Activity::listening("the code")
                                         ];
    let activityArrayLength = activityArray.len();
    let mut activityNumber: usize = rng.gen_range(0, activityArrayLength);
    unsafe
    {
        while activityNumber == lastActivityNumber
        {
            activityNumber = rng.gen_range(0, activityArrayLength);
        }
        context.set_activity(activityArray[activityNumber].clone());
        lastActivityNumber = activityNumber;
    }
    println!("New activity set!");
}


fn isBotGod(msg: Message, context: Context) -> bool
{
    let mut botGod = false;
    let guildID: GuildId = msg.guild_id.unwrap();

    if let Some(arc) = msg.guild_id.unwrap().to_guild_cached(&context.cache)
    {
        if let Some(role) = arc.read().role_by_name("BotGod")
        {
        botGod = msg.author.has_role(
                                     context.clone(), 
                                     Guild::get(
                                                context.http.clone(),
                                                guildID
                                                ).unwrap(),
                                     role
                                     ).unwrap();
        }        
    }
    return botGod;
}
