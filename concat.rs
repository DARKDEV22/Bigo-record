use std::fs;
use std::process::Command;
use std::fs::File;
use std::io::Write;

pub fn combined_videos(folder_name: &str) -> Result<(), std::io::Error>{
    let dir_path: Result<Vec<_>, _> = fs::read_dir(format!("./{}", folder_name))?.collect();
    let file_names: Vec<String> = dir_path?
                                .iter()
                                .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
                                .collect();
    // file list
    let file_names: Vec<&str> = file_names.iter().map(|name| name.as_str()).collect();

    //write .txt for path list
    let mut file = File::create(format!("{}.txt", folder_name))?;
    for file_name in file_names {
        writeln!(file, "file '{}/{}'", folder_name, file_name)?;
    }

    // combined files
    let output = Command::new("ffmpeg")
                                .args(["-f", "concat", "-safe", "0", "-i", &format!("{}.txt", folder_name), "-c", "copy", &format!("{} - concated.mp4", folder_name)])
                                .output()
                                .expect("failed to execute process");

    let _ = output.stdout;
    Ok(())
}

fn main() {
    _ = combined_videos("Numwarn12345678");
}