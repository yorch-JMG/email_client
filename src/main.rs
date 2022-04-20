use colored::Colorize;
use pyo3::{types::PyModule, PyResult, Python};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::io::stdin;

#[derive(Serialize, Deserialize)]
struct Email {
    emailId: i32,
    to: String,
    from: String,
    title: String,
    emailBody: String,
    read: bool,
}

#[derive(Serialize, Deserialize)]
struct SendEmail {
    to: String,
    from: String,
    title: String,
    emailBody: String,
}

fn status(read: bool) -> String {
    let read_color: String;
    if read {
        read_color = "Read".green().to_string();
        return read_color;
    }
    read_color = "Read now".red().to_string();
    return read_color;
}

fn get_email_data() -> SendEmail {
    let mut destination = String::new();
    let mut sent_from = String::new();
    let mut email_title = String::new();
    let mut email_body = String::new();
    loop {
        println!("To:");
        stdin().read_line(&mut destination).expect("No value found");
        if !destination.is_empty() {
            break;
        }
        println!("Destination field needs a value:");
    }
    loop {
        println!("From:");
        stdin().read_line(&mut sent_from).expect("No value found");
        if !sent_from.is_empty() {
            break;
        }
        println!("From field needs a value:");
    }
    loop {
        println!("Title:");
        stdin().read_line(&mut email_title).expect("No value found");
        if !email_title.is_empty() {
            break;
        }
        println!("Title field needs a value:");
    }
    loop {
        println!("Email body:");
        stdin().read_line(&mut email_body).expect("No value found");
        if !email_body.is_empty() {
            break;
        }
        println!("Body field needs a value:");
    }

    return SendEmail {
        to: destination,
        from: sent_from,
        title: email_title,
        emailBody: get_py_encryption(&email_body).unwrap(),
    };
}
pub fn get_py_decryption(email_body: &String) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let encryption = PyModule::from_code(
            py,
            r#"
def vigenere(text: str, key: str, encrypt=True):
    result = ''

    for i in range(len(text)):
        letter_n = ord(text[i])
        key_n = ord(key[i % len(key)])

        if encrypt:
            value = (letter_n + key_n) % 1114112
        else:
            value = (letter_n - key_n) % 1114112

        result += chr(value)

    return result

def vigenere_decrypt(text: str, key: str):
    return vigenere(text=text, key=key, encrypt=False)
                "#,
            "vigenere.py",
            "vigenere",
        )?;
        let decryption_result :String = encryption.getattr("vigenere_decrypt")?.call1((email_body,"hola".to_string()))?.extract()?;
        println!("{}", decryption_result);

        Ok(decryption_result)
    })
}

pub fn get_py_encryption(email_body: &String) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let encryption = PyModule::from_code(
            py,
            r#"
def vigenere(text: str, key: str, encrypt=True):
    result = ''

    for i in range(len(text)):
        letter_n = ord(text[i])
        key_n = ord(key[i % len(key)])

        if encrypt:
            value = (letter_n + key_n) % 1114112
        else:
            value = (letter_n - key_n) % 1114112

        result += chr(value)

    return result


def vigenere_encrypt(text: str, key: str):
    return vigenere(text=text, key=key, encrypt=True)
                "#,
            "vigenere.py",
            "vigenere",
        )?;
        let encryption_result :String = encryption.getattr("vigenere_encrypt")?.call1((email_body,"hola".to_string()))?.extract()?;
        println!("{}", encryption_result);

        Ok(encryption_result)
    })
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
        if user_input.contains("1") {
            println!();
            println!("All emails:");
            let response = reqwest::get("http://localhost:3000/emails").await?;
            let emails: Vec<Email> = response.json().await?;
            for (i, email) in emails.iter().enumerate() {
                let mut title = email.title.clone();
                if let Some('\n') = title.chars().next_back() {
                    title.pop();
                }
                let line = format!("{}. {} {}", &i + 1, title, status(email.read));
                println!("{}", line)
            }
            println!();
        }
        if user_input.contains("2") {
            let new_email = get_email_data();
            let _send_email: Email = reqwest::Client::new()
                .post("http://localhost:3000/email/send")
                .json(&new_email)
                .send()
                .await?
                .json()
                .await?;
            println!("Email sent!");
        }
        if user_input.contains("3") {
            println!();
            println!("All emails:");
            let response = reqwest::get("http://localhost:3000/emails").await?;
            let emails: Vec<Email> = response.json().await?;
            for (i, email) in emails.iter().enumerate() {
                let mut title = email.title.clone();
                if let Some('\n') = title.chars().next_back() {
                    title.pop();
                }
                let line = format!("{}. {} {}", &i + 1, title, status(email.read));
                println!("{}", line)
            }
            println!();
            println!("What email to read?");
            let mut user_input = String::new();
            stdin().read_line(&mut user_input).expect("No value found");
            if !user_input.is_empty() {
                let mut id = user_input.clone();
                if let Some('\n') = id.chars().next_back() {
                    id.pop();
                }
                let localhost = "http://localhost:3000";
                let url = format!("{}/{}", localhost, id);
                let response = reqwest::get(url).await?;
                let email: Email = response.json().await?;
                println!("Title: {}", email.title);
                println!("To: {}", email.to);
                println!("From: {}", email.from);
                println!("Body: {}", email.emailBody);
            } else {
                println!("Not a valid value")
            }
        }     
    }
}
