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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:3000/emails")
        .await?
        .json::<Vec<Email>>()
        .await?;
    println!("{}", resp[0].emailBody);
    Ok(())
}
