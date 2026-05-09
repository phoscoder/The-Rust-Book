use tokio::time::sleep;
use std::time::Duration;


async fn fetch_user() {
    println!("Fetching user...");
}

async fn logging() {
    println!("Logging event...");
}

async fn load_config() -> () {
    println!("Loading config...");
    ()
}

async fn connect_to_db() -> () {
    println!("Connecting to database...");
    ()
}


#[tokio::main]
async fn main() {

    // await sequentially
    // fetch_user().await;
    // println!("User fetched!");


    // fire and forget tasks 
    tokio::spawn(fetch_user());
    tokio::spawn(logging());

    // run futures in a single task 
    let (_, _) = tokio::join!(load_config(), connect_to_db());

    
    // In aysnc rust you can also race two futures together
    // tokio::select! poll both futures and return the first one that completes first
    // Once one future completes, the other is cancelled and their state is cleaned
    tokio::select! {
        _ = fetch_user() => {
            
        }

        // implict cancellation
        _ = sleep(Duration::from_secs(5)) => {
            // loser
        }
    }

    // spawn -> create a new task 
    // await -> run futures sequentially
    // join! -> run futures concurrently
    // select! -> run futures concurrently and return the first one that completes first

    
}
