use xmly::tasks::{XmlyApp};
use xmly::config::CONFIG;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for token in CONFIG.token_list.iter() {
        let mut app = XmlyApp::new(token.to_string());
        let _res = match app.run().await {
            Ok(()) => println!(), 
            Err(_err) => println!("error")
        };
    }
    Ok(())
}
