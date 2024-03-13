use serde::{Deserialize, Serialize};
use std::{env, fs, time::{SystemTime, UNIX_EPOCH}};
use serde_json;

#[derive(Serialize, Deserialize)] // Implement Serialize and Deserialize traits
struct Monitor {
    name: String,
    script: Option<String>,
    monitor_id: Option<u32>,
    #[serde(default)]
    result: Option<Result>, // Include Result as an option
    code: String,
}

// #[derive(Serialize)] // Implement Serialize trait
#[derive(Serialize, Deserialize)]  //Implement Serialize and Deserialize traits
struct Result {
    value: i32,
    processed_at: u64,
}

#[derive(Serialize, Deserialize)] // Implement Serialize and Deserialize traits
struct Monitors {
    monitors: Vec<Monitor>,
}

fn main() {
    //  Process command line arguments
    let args: Vec<String> = env::args().collect();
    let mut monitor_file = None;
    for i in 0..args.len() {
        if args[i] == "-monitorFile" && i < args.len() - 1 {
            monitor_file = Some(args[i + 1].clone());
            break;
        }
    }

    let monitor_file = match monitor_file {
        Some(path) => path,
        None => {
            eprintln!("Error: Missing required argument -monitorFile");
            return;
        }
    };

    // Read JSON file and deserialize it into data structure
    let file_content = match fs::read_to_string(&monitor_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", monitor_file, err);
            return;
        }
    };

    let mut monitors_data: Monitors = match serde_json::from_str(&file_content) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing JSON data: {}", err);
            return;
        }
    };

    //  Generate and assign results for each monitor
    for monitor in &mut monitors_data.monitors {
        monitor.result = Some(Result {
            value: rand::random::<i32>(),
            processed_at: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs(),
        });
    }

    // Convert results to JSON
    let json_results = match serde_json::to_string_pretty(&monitors_data) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Error converting results to JSON: {}", err);
            return;
        }
    };

    println!("{}", json_results);
}
