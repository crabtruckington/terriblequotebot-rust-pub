#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(dead_code)]

use std::*;
use rand::prelude::*;
use postgres::{Client as pclient, NoTls, Row};
#[path = "configs.rs"]
mod configs;

// quotes table definition
// CREATE TABLE quotes ( id INT GENERATED ALWAYS AS IDENTITY, quote VARCHAR NOT NULL);


pub fn getTotalQuoteValueFromDatabase() -> i32
{
    println!("Starting to retrieve total quotes from database...");
    println!("Connecting to database...");
    let mut postgresclient = pclient::connect(configs::POSTGRESCONNSTRING, NoTls).unwrap();    
    println!("Connection established, fetching max quote number...");

    let row: Row = postgresclient.query_one("SELECT max(id) from quotes", &[]).unwrap();
    let totalQuotes: i32 = row.get(0);
    println!("There are {} total quotes in the database!", totalQuotes.to_string());
    return totalQuotes;
}

pub fn getRandomQuoteFromDatabase() -> String
{
    println!("Generating random number based on total quotes...");
    let mut rng = thread_rng();
    let randQuote = rng.gen_range(1, getTotalQuoteValueFromDatabase() + 1);
    println!("Fetching quote {}", randQuote);

    println!("Connecting to database...");
    let mut postgresclient = pclient::connect(configs::POSTGRESCONNSTRING, NoTls).unwrap();
    println!("Connection established!");
    let row: Row = postgresclient.query_one("SELECT quote FROM quotes WHERE id = $1", &[&randQuote]).unwrap();
    let returnedQuote: &str = row.get(0);
    println!("Got the following quote from the database: {}", returnedQuote);
    println!("Formatting the quote for output...");
    let result = format!("Quote {}: {}", randQuote.to_string(), returnedQuote.to_string());
    println!("Quote formatted, returning...");
    return result;
}

pub fn getSpecificQuoteFromDatabase(quoteNumber: &i32) -> String
{
    let totalQuotes = getTotalQuoteValueFromDatabase();
    if (quoteNumber < &1 || quoteNumber > &totalQuotes)
    {
        return format!("You have not provided a valid selection! Valid selections are from 1 - {}!", totalQuotes);
    }
    println!("Connecting to database...");
    let mut postgresclient = pclient::connect(configs::POSTGRESCONNSTRING, NoTls).unwrap();
    println!("Connection established, fetching quote {}...", quoteNumber);

    let row: Row = postgresclient.query_one("SELECT quote FROM quotes WHERE id = $1", &[&quoteNumber]).unwrap();
    let returnedQuote: &str = row.get(0);
    println!("Got the following quote from the database: {}", returnedQuote);
    println!("Formatting the quote for output...");
    let result = format!("Quote {}: {}", quoteNumber.to_string(), returnedQuote.to_string());
    println!("Quote formatted, returning...");
    return result;
}

pub fn findQuotesByText(searchTerm: String) -> String
{
    println!("Connecting to database...");
    let mut postgresclient = pclient::connect(configs::POSTGRESCONNSTRING, NoTls).unwrap();
    println!("Connection established, searching for quotes with text '{}'...", searchTerm);

    let mut quoteString: String = "".to_string();
    for row in postgresclient.query("select id from quotes where quote like $1", &[&format!("%{}%", searchTerm)]).unwrap()
    {
        let id: i32 = row.get(0);
        quoteString.push_str(&format!("{}, ", id));
    }
    return quoteString;
}

pub fn addQuoteToDatabase(quoteToAdd: String) -> i32
{
    println!("Connecting to database...");
    let mut postgresclient = pclient::connect(configs::POSTGRESCONNSTRING, NoTls).unwrap();
    println!("Connection established, adding quote...");

    postgresclient.execute("INSERT INTO quotes (quote) VALUES ($1)", &[&quoteToAdd]).unwrap();
    let newQuoteID = getTotalQuoteValueFromDatabase();
    println!("New quote successfully added as quote #{}", newQuoteID);
    
    return newQuoteID;
}

// This function deletes a row from the database, by ID, and then
// table swaps the quotes table. This is a workaround for the fact
// there is no elegant way to renumber the ident columns in postgres
// if you are using a database where this is possible (mssql for example)
// you can change the tableswap query to a re-ident query, since 
// the whole goal is just to not have gaps in the sequence.
// NOTE: truncate does NOT re-ident a postgres table, dont bother trying.
pub fn deleteQuoteFromDatabase(quoteNumber: &i32) -> String
{
    let totalQuotes = getTotalQuoteValueFromDatabase();
    if (quoteNumber < &1 || quoteNumber > &totalQuotes)
    {
        return format!("You have not provided a valid selection! Valid selections are from 1 - {}!", totalQuotes);
    }
    println!("Connecting to database...");
    let mut postgresclient = pclient::connect(configs::POSTGRESCONNSTRING, NoTls).unwrap();
    println!("Connection established, deleting quote {}...", quoteNumber);

    postgresclient.execute("DELETE FROM quotes WHERE id = $1", &[&quoteNumber]).unwrap();
    println!("Quote {} deleted, swapping tables to reset IDs...", quoteNumber);

    let statement = 
    "CREATE TABLE quotestmp ( id INT GENERATED ALWAYS AS IDENTITY, quote VARCHAR NOT NULL );
    INSERT INTO quotestmp (quote) SELECT quote FROM quotes;
    DROP TABLE quotes;
    CREATE TABLE quotes ( id INT GENERATED ALWAYS AS IDENTITY, quote VARCHAR NOT NULL );
    INSERT INTO quotes (quote) SELECT quote FROM quotestmp;
    DROP TABLE quotestmp;"
    ;
    
    postgresclient.batch_execute(&statement).unwrap();
    println!("Tables swapped!");

    let totalQuotes = getTotalQuoteValueFromDatabase();

    return format!("Quote {} deleted! New quote range from 1 - {}", quoteNumber, totalQuotes);
}