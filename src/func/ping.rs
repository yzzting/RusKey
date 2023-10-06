use crate::Store;
use crate::func::stream::Client;

pub async fn ping(command: &str, state: &Store) {
    let mut client = Client::new(&state.url).await.unwrap();
    match client.send_command(command).await {
        Ok(response) => {
            println!("Received: {}", response);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}