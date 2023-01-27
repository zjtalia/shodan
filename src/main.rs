use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use config::{Config, FileFormat};
use reqwest::Response;
use serde_json::{to_string_pretty, Value};

fn load_settings() -> Result<Config, config::ConfigError> {
    let config_file = config::File::new("Settings.toml", FileFormat::Toml);
    let config = Config::builder().add_source(config_file);
    config.build()
}

fn read_ip_addresses(file_path: &PathBuf) -> Vec<String> {
    let ip_list_file: File = File::open(file_path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(ip_list_file);
    let mut ip_list: Vec<String> = Vec::new();

    loop {
        let mut line = String::with_capacity(16);
        let bytes_read = reader.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        } else {
            ip_list.push(line.trim_end().to_string())
        }
    }
    ip_list
}

#[tokio::main]
async fn shodan_ip_lookup(
    ip_address: &String,
    api_key: &String,
    history: &String,
    minify: &String,
) -> Value {
    let url: String = format!("https://api.shodan.io/shodan/host/{ip_address}?key={api_key}&history={history}&minify={minify}");
    let res: Response = reqwest::get(url).await.unwrap();

    if res.status().as_u16() == 401 {
        panic!("Error: Invalid API Key")
    };

    res.json::<Value>().await.unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let config: Config = load_settings().unwrap();
    let api_key: String = config.get("api_key").unwrap();
    let history: String = config.get_string("history").unwrap();
    let minify: String = config.get_string("minify").unwrap();

    let mut ip_data: HashMap<String, Value> = HashMap::new();

    let output_file_path = PathBuf::from("ip_results.json");
    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);

    for arg in env::args().skip(1) {
        let file_path: PathBuf = PathBuf::from(&arg);
        let ip_addresses: Vec<String> = read_ip_addresses(&file_path);
        for ip_address in ip_addresses {
            if !ip_address.is_empty() && !ip_data.contains_key(&ip_address) {
                _ = ip_data.insert(
                    ip_address.clone(),
                    shodan_ip_lookup(&ip_address, &api_key, &history, &minify),
                );
            }
        }
    }
    writer.write_all(to_string_pretty(&ip_data)?.as_bytes())?;
    Ok(())
}
