use serde::{Deserialize, Serialize};
use std::{
    env,
    fs,
    io::{self, Write},
    // path::Path,
    sync::{Arc, Mutex},
    thread,
    time::{SystemTime, UNIX_EPOCH, Duration},
};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Monitor {
    name: String,
    script: Option<String>,
    monitor_id: Option<u32>,
    #[serde(default)]
    result: Option<Result>, 
    code: String,
}

#[derive(Serialize, Deserialize)]
struct Result {
    value: i32,
    processed_at: u64,
}

#[derive(Serialize, Deserialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}

fn process_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "-monitorFile" && i < args.len() - 1 {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn read_monitors_json(file_path: &str) -> io::Result<Monitors> {
    let file_content = fs::read_to_string(file_path)?;
    let monitors_data = serde_json::from_str(&file_content)?;
    Ok(monitors_data)
}

fn update_monitor_results(monitors: &mut Monitors) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    for monitor in &mut monitors.monitors {
        let result = Result {
            value: rand::random::<i32>(),
            processed_at: now,
        };
        monitor.result = Some(result);
    }
}

fn store_monitors(monitors: &Monitors) -> io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let formatted_time = timestamp_to_human_readable(timestamp);
    let filename = format!("{}_monitors.json", formatted_time);
    let data = serde_json::to_string_pretty(monitors)?;
    let mut file = fs::File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn timestamp_to_human_readable(timestamp: u64) -> String {
    let dt = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = chrono::DateTime::<chrono::Utc>::from(dt);
    datetime.format("%Y-%m-%d_%H-%M-%S").to_string()
}

fn process_monitors(monitors: Monitors) -> io::Result<()> {
    let monitors_arc = Arc::new(Mutex::new(monitors));
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    loop {
        update_monitor_results(&mut monitors_arc.lock().unwrap());
        thread::sleep(Duration::from_secs(30));

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        if current_time - start_time >= 300 {
            break;
        }

        store_monitors(&monitors_arc.lock().unwrap())?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let monitor_file = match process_args() {
        Some(path) => path,
        None => {
            eprintln!("Error: Missing required argument -monitorFile");
            return Ok(());
        }
    };

    let monitors_data = read_monitors_json(&monitor_file)?;
    process_monitors(monitors_data)?;

    Ok(())
}
