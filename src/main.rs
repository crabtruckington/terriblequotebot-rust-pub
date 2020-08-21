#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(dead_code)]

use std::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;

mod configs;
mod commandparser;
mod sqltools;

struct Handler;
impl EventHandler for Handler
{
    fn message(&self, context: Context, msg: Message)
    {
        let isCommand = (msg.content.starts_with(configs::DISCORDPREFIX) && msg.author.bot == false);

        if (isCommand)
        {
            let unwrapArgs = (msg.content.splitn(2, " ").count() > 1);
            let mut cmdArgs = msg.content.splitn(2, " ");
            let command = cmdArgs.next().unwrap().to_lowercase();
            let mut args = "";
            if (unwrapArgs == true)
            {
                args = cmdArgs.next().unwrap();
            }
            commandparser::parseCommand(command, args.trim().to_string(), context, msg);
        }
    }

    fn ready(&self, context: Context, _ready: Ready)
    {
        println!("TerribleQuoteBot, written in terrible Rust, is starting, setting activity...");
        commandparser::setActivityType(context.clone());
        println!("Activity set! Getting Total Quotes value...");
        sqltools::getTotalQuoteValueFromDatabase();
        println!("Startup complete! Waiting for commands...");
        thread::spawn(move || {setNewActivityOnInterval(context.clone())});
    }
}

fn setNewActivityOnInterval(context: Context)
{
    //set a new activty every X seconds
    println!("We are going to set an activity in 1800 seconds");
    let interval = time::Duration::from_secs(1800);
    loop
    {
        thread::sleep(interval);
        commandparser::setActivityType(context.clone());
        println!("Activity set! We are going to set another activity in 1800 seconds");
    }
}

fn main()
{
    let mut client = Client::new(configs::DISCORDTOKEN, Handler)
        .expect("Couldnt create new client!");

    if let Err(why) = client.start()
    {
        println!("Client error: {}", why);
    }
}