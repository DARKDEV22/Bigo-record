extern crate reqwest;
use reqwest::header;
use std::env;
use std::thread;
use std::time::{Duration, SystemTime};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::io;

fn get_m3u8(user_id: &str) -> Result<String, Box<dyn std::error::Error>> {

    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.bigo.tv".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.6".parse().unwrap());
    headers.insert("if-none-match", "\"2232d-VHn+zGI8e88vY+/Y2VFdd20rpsE\"".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Brave\";v=\"116\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?1".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Android\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get(&format!("https://www.bigo.tv/en/{}", user_id))
        .headers(headers)
        .send()?
        .text()?;

    Ok(res)
}


fn get_ts4(m3u8_url: &str) -> Result<String, Box<dyn std::error::Error>> {

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.6".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Origin", "https://www.bigo.tv".parse().unwrap());
    headers.insert("Referer", "https://www.bigo.tv/".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "cross-site".parse().unwrap());
    headers.insert("Sec-GPC", "1".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Brave\";v=\"116\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?1".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Android\"".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get(m3u8_url)
        .headers(headers)
        .send()?
        .text()?;

    Ok(res)
}

fn create_dir(dir_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir(dir_name)?;
    Ok(())
}

async fn download_file(download_url: &str, save_path: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>{
    let response = reqwest::get(download_url).await?;
    let file_size = response.content_length().unwrap_or(0);
    // if it found video not "not found" page
    if file_size > 500 {
        let mut file = std::fs::File::create(save_path)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
    }
    Ok(file_size)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let user_id: String;

    if args.len() != 2 {
        // let user_id_input = "859931431".to_string();
        let mut user_id_input = String::new();
        println!("\nEnter bigo user_id_input :");
        io::stdin().read_line(&mut user_id_input).expect("Failed to read");
        user_id = user_id_input.trim().to_string();
    } else {
        user_id = args[1].to_string();
    }
    
    // find previous video_id 
    let mut previous_id = 0;
    let path = Path::new(&user_id);
    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap();
        let file_names: Vec<String> = entries
                                    .filter_map(|entry| {
                                        let entry = entry.ok()?;
                                        let file_name = entry.file_name();
                                        Some(file_name.to_string_lossy().to_string())
                                    })
                                    .collect();
        let numbers: Vec<i32> = file_names
                                .iter()
                                .filter_map(|file_name| {
                                    let parts: Vec<&str> = file_name.split('.').collect();
                                    parts.first()?.parse().ok()
                                })
                                .collect();

        if let Some(max_number) = numbers.iter().max() {
            previous_id = *max_number;
        } 
    }

    let user_id = user_id.as_str();
    let delay_sec = 2;
    let sec_interval = Duration::from_millis(200);
    let retry_interval = Duration::from_millis(250);
    let max_timing = Duration::from_secs(60*11); // run 11minutes for some network interrupt
    let start_time = SystemTime::now();

    // download section
    println!("\n[INFO] Ctrl + C : stop recording.");
    println!("\nRecording: {}", user_id);
    while start_time.elapsed().unwrap() < max_timing {
        match get_m3u8(user_id) {
            Ok(response) => {
                // get m3u8_url 
                let source: Vec<&str> = response
                                        .split("video_tag_show")
                                        .nth(1)
                                        .unwrap_or("")
                                        .split("video")
                                        .nth(1)
                                        .unwrap_or("")
                                        .split("source")
                                        .collect();
                let base_url: Vec<&str> = source[1]
                                        .split("\\")
                                        .next()
                                        .unwrap_or("")
                                        .split("https")
                                        .nth(1)
                                        .unwrap_or("")
                                        .split("m3u8")
                                        .collect(); 
                let m3u8_url = format!("https{}m3u8", base_url[0]);
                let base: Vec<&str> = m3u8_url.split("/").collect();
                let base_url = format!("https://{}", base[2]);

                // m3u8 response -> ts4 download url
                match get_ts4(&m3u8_url){
                    Ok(response) => {
                        // create directory
                        let _dir = create_dir(user_id);
                        let r: Vec<&str> = response.split("\n").collect();
                        let base_id: Vec<&str> = r[2].split(":").collect();
                        if let Ok(start_id) = base_id[1].parse::<i32>() {
                            let mut start_id: i32 = start_id + delay_sec;
                            let base_nums: Vec<&str> = r[6].split("_").collect();
                            let second_num: Vec<&str> = base_nums[1].split("&").collect();
                            let base_nums_vec: Vec<&str> = vec![base_nums[0], second_num[1]];
                            // println!("{:?} {:?} {:?}", start_id, base_url, base_nums_vec);

                            let mut n_not_complete: u8 = 0;
                            let limit_download_attempt: u8 = 36;
                            let may_be_next_id: u8 = 15;
                            let mut n_videos = 1;
                            while  n_not_complete <= limit_download_attempt {
                                let download_url = format!("{}/{}_{}&{}&0.ts", base_url, base_nums_vec[0], start_id, base_nums_vec[1]);
                                let save_path = format!("{}/{}.ts", user_id, start_id + previous_id);
                                println!("id: {}: {} | {}", user_id, start_id, n_videos);
                                if n_videos%40 == 0 {
                                    println!(" -- estimate {:.2} minutes.", (n_videos*2) as f32/60.0);
                                }
                                let result = download_file(&download_url, &save_path).await;
                                // check is valid file ?
                                let file_size = match result {
                                    Ok(size) => size,
                                    Err(err) => {
                                        eprintln!("Error content length: {}", err);
                                        0
                                    }
                                };

                                if file_size < 500 {
                                    n_not_complete += 1;
                                    thread::sleep(retry_interval);
                                    if n_not_complete > may_be_next_id {
                                        start_id += 1;
                                    }
                                }
                                else {
                                    n_not_complete = 0;
                                    start_id += 1;
                                    n_videos += 1;
                                    thread::sleep(sec_interval);
                                }
                                
                            }
                            println!("[LIVE] : {:?}", start_time.elapsed().unwrap());

                        } else {
                            println!("Conversion failed");
                        }
                    },
                    Err(_) => {
                        println!("\nUser id: {} not found", user_id);
                    }
                }    
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
