use crate::func::stream::Client;
use crate::Store;

pub async fn set_string(original_command: &str, state: &Store) {    
    let mut client = Client::new(&state.url).await.unwrap();
    match client.send_command(original_command).await {
        Ok(response) => {
            println!("Received: {}", response);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}