use crate::func::stream::Client;
use crate::Store;

pub async fn get_string(origin_command: &str, state: &Store) {
    let mut client = Client::new(&state.url).await.unwrap();
    match client.send_command(origin_command).await {
        Ok(response) => {
            println!("Received: {}", response);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}