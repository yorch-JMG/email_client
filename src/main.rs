use std::{io::stdin};
use colored::Colorize;
use reqwest::Error;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Email {
    emailId : i32,
    to: String,
    from: String,
    title: String,
    emailBody: String,
    read: bool
}

fn print_status(read: bool){
    let mut read_color : String;
    if read {
        read_color = "Read".green().to_string();
        println!("Status: {}",read_color)
    }
    read_color = "Read now".red().to_string();
    println!("Status: {}",read_color)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        println!("Welcome to this email client, please select an option from below:");
        println!("1. Get all emails");
        println!("2. Send an email");
        println!("3. Read an email");

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("No value found");
            if user_input.contains("1"){
                let response = reqwest::get("http://localhost:3000/emails").await?;
                let emails: Vec<Email> = response.json().await?;
                for (i, email) in emails.iter().enumerate() {
                    println!("{}. {}", &i+1, email.title);
                    print_status(email.read);
                }
            }
            if user_input.contains("1"){
            }
            if user_input.contains("1"){
            }
            else {
                break;
            }
        };
        Ok(())
    }
