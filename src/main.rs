extern crate google_sheets4 as sheets4;
use sheets4::Sheets;


mod auth;
mod config;
mod http_client;
mod sheets;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);

    let result = sheets::write(&hub, &config).await;

    match result {
        Err(e) => println!("{}", e),
        Ok((_, response)) => {
           

            println!(
                "Success: {:?}", response
            );
        }
    }

    /*
     
    match result {
        Err(e) => println!("{}", e),
        Ok((_, spreadsheet)) => {
           

            println!(
                "Success: {:?}",
                spreadsheet
                    .values
                    .unwrap()
                    .into_iter()
                    
            );
        }
    }
    */
}
