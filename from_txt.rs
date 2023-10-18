use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;
use std::thread;
use std::time::Duration;
use tokio::time;

fn process_user_ids() -> io::Result<()> {
    let file = File::open("users_id.txt")?;
    let reader = io::BufReader::new(file);

    // Create a vector to store thread handles
    let mut handles = Vec::new();

    for user_id in reader.lines() {
        let user_id = user_id?;

        // Spawn a new thread for each user ID
        let handle = thread::spawn(move || {
            let status = Command::new("main.exe")
                .arg(&user_id)
                .status();

            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("main.exe executed successfully for user ID: {}", user_id);
                    } else {
                        eprintln!("main.exe failed with exit code {} for user ID: {}", exit_status.code().unwrap_or(-1), user_id);
                    }
                }
                Err(err) => {
                    eprintln!("Error running main.exe for user ID {}: {:?}", user_id, err);
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    loop {
        process_user_ids()?;

        // Sleep for 11:20 minutes 
        let interval = Duration::from_secs(680);
        time::sleep(interval).await;
    }
}