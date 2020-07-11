use fortnite::Client;
use std::io::stdin;
use std::error::Error;

#[tokio::test]
async fn run_test() -> Result<(), Box<dyn Error>> {
    let mut auth_code = String::new();
    let mut client = Client::new();
    stdin().read_line(&mut auth_code).expect("Failed to read line");
    if cfg!(windows) {
        let len = auth_code.len();
        auth_code.truncate(len - 2);
    }
    else {
        let len = auth_code.len();
        auth_code.truncate(len - 1);
    } // This removes the \n from the end of the string
    client.authenticate(auth_code).await?;
    client.connect_xmpp();
    Ok(())
}

#[test]
fn lol() {
    unimplemented!();
}