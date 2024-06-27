use chrono::prelude::*;
use std::fs::create_dir;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn help_command() {
    eprintln!("Use: narma [command]");
    eprintln!("Commands:");
    eprintln!("  cache [nargo command]           Cache ACIR from output of nargo");
    eprintln!("  diff [timestamp1] [timestamp2]  Check difference between ACIR's");
    eprintln!("  list                            List all available ACIR's");
    eprintln!("  clean                           Clean available cache");
    eprintln!("  help                            Show help");
    
    return;
}

fn cache_command(args: &[String]) {
    let cache_dir = Path::new("./.narma-cache");
    if !cache_dir.exists() {
        create_dir(cache_dir).unwrap();
    }

    let output = Command::new("nargo")
        .args(args)
        .arg("--print-acir")
        .output()
        .unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let cache_file = cache_dir.join(format!("{}.acir", timestamp));
    std::fs::write(cache_file, &output.stdout).unwrap();
    print!("{}", String::from_utf8(output.stderr).unwrap());
}

fn diff_command(args: &[String]) {
    let cache_dir = Path::new("./.narma-cache");

    match args.len() {
        1 => {
            let timestamp1 = args[0].parse::<u64>().unwrap();
            let file1 = cache_dir.join(format!("{}.acir", timestamp1));

            if !file1.exists() {
                eprintln!("Error: ACIR file for timestamp {} not found.", timestamp1);
                return;
            }

            let mut latest_time = 0;
            let mut latest_file = None;
            for file in std::fs::read_dir(cache_dir).unwrap() {
                let file = file.unwrap();
                let name = file.file_name();
                let name = name.to_str().unwrap();

                if name.ends_with(".acir") {
                    let timestamp = name[0..name.len() - 5].parse::<u64>().unwrap();

                    if timestamp > latest_time {
                        latest_time = timestamp;
                        latest_file = Some(cache_dir.join(name.to_string()));
                    }
                }
            }

            if let Some(file2) = latest_file {
                let output = Command::new("diff")
                    .arg(file1.as_os_str().to_str().unwrap())
                    .arg(file2.as_os_str().to_str().unwrap())
                    .output()
                    .unwrap();

                print!("{}", String::from_utf8(output.stdout).unwrap());
            }
            return;
        }
        2 => {
            let timestamp1 = args[0].parse::<u64>().unwrap();
            let file1 = cache_dir.join(format!("{}.acir", timestamp1));

            if !file1.exists() {
                eprintln!("Error: ACIR file for timestamp {} not found.", timestamp1);
                return;
            }

            let timestamp2 = args[1].parse::<u64>().unwrap();
            let file2 = cache_dir.join(format!("{}.acir", timestamp2));

            if !file2.exists() {
                eprintln!("Error: ACIR file for timestamp {} not found.", timestamp2);
                return;
            }
            let output = Command::new("diff")
                .arg(file1.as_os_str().to_str().unwrap())
                .arg(file2.as_os_str().to_str().unwrap())
                .output()
                .unwrap();
            println!("{}", String::from_utf8(output.stdout).unwrap());
        }
        _ => help_command(),
    }
}

fn list_command() {
    let cache_dir = Path::new("./.narma-cache");
    if !cache_dir.exists() {
        eprintln!("Cache directory not found.");
        return;
    }

    for file in std::fs::read_dir(cache_dir).unwrap() {
        let file = file.unwrap();
        let name = file.file_name();
        let name = name.to_str().unwrap();
        if name.ends_with(".acir") {
            let timestamp = name[0..name.len() - 5].parse::<i64>().unwrap();

            let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
            let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

            println!("{}: {}", newdate, name);
        }
    }
}

fn clean_command() {
    let cache_dir = Path::new("./.narma-cache");
    if !cache_dir.exists() {
        eprintln!("Cache directory not found.");
        return;
    }

    std::fs::remove_dir_all(cache_dir).unwrap();
    std::fs::create_dir(cache_dir).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help_command();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "cache" => cache_command(&args[2..]),
        "diff" => diff_command(&args[2..]),
        "list" => list_command(),
        "clean" => clean_command(),
        _ => help_command(),
    }
}
