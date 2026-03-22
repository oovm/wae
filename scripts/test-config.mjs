// Simple test script to verify the new ConfigLoader implementation
import fs from 'fs';
import path from 'path';

// Create a temporary TOML config file
const configContent = `
[server]
port = 8080
host = "localhost"

[database]
url = "mongodb://localhost:27017"
name = "testdb"
`;

const configPath = path.resolve('./test-config.toml');
fs.writeFileSync(configPath, configContent);

// Create a simple Rust test program
const testRustContent = `
use serde::Deserialize;
use wae_config::{ConfigLoader, load_config, from_env};

#[derive(Debug, Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    port: u16,
    host: String,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    name: String,
}

fn main() {
    println!("Testing ConfigLoader...");
    
    // Test 1: Load config from file
    let config: Config = load_config("test-config.toml", "APP_").unwrap();
    println!("Loaded config: {:?}", config);
    
    // Test 2: Load config with defaults
    let defaults = Config {
        server: ServerConfig {
            port: 3000,
            host: "0.0.0.0".to_string(),
        },
        database: DatabaseConfig {
            url: "mongodb://localhost:27017".to_string(),
            name: "defaultdb".to_string(),
        },
    };
    
    let config_with_defaults: Config = ConfigLoader::new()
        .with_toml("test-config.toml")
        .with_defaults(&defaults)
        .extract()
        .unwrap();
    println!("Config with defaults: {:?}", config_with_defaults);
    
    println!("All tests passed!");
}
`;

const testRustPath = path.resolve('./test-config.rs');
fs.writeFileSync(testRustPath, testRustContent);

console.log('Test files created successfully!');
console.log('To run the test, execute: rustc test-config.rs --extern wae_config=target/debug/libwae_config.rlib');
console.log('Then run: ./test-config.exe');
