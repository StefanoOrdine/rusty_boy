use std::env;
use std::process::{Command, Stdio};

fn main() {
    println!("🚀 Launching DMG-01 book...");
    
    // Get the project root directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dmg01_dir = current_dir.join("resources").join("DMG-01").join("book");
    
    println!("📁 Project root: {}", current_dir.display());
    println!("📚 DMG-01 book directory: {}", dmg01_dir.display());
    
    // Check if DMG-01 book directory exists
    if !dmg01_dir.exists() {
        eprintln!("❌ Error: DMG-01 book directory not found at {}", dmg01_dir.display());
        std::process::exit(1);
    }
    
    // Change to DMG-01 book directory
    env::set_current_dir(&dmg01_dir).expect("Failed to change to DMG-01 book directory");
    
    // Check if mdbook is available
    if !command_exists("mdbook") {
        eprintln!("❌ Error: mdbook is not installed");
        eprintln!("Please install mdbook: cargo install mdbook");
        std::process::exit(1);
    }
    
    // Find an available port (starting from 3100 to avoid conflict with Pandocs)
    let port = find_available_port(3100);
    
    println!("🌐 Starting mdbook server on port {}...", port);
    println!("📖 The book will be available at: http://localhost:{}", port);
    println!("📘 Book title: DMG-01: How to Emulate a Game Boy");
    println!("🔄 The server will watch for file changes and auto-reload");
    println!();
    println!("Press Ctrl+C to stop the server");
    println!();
    
    // Start the mdbook server
    let status = Command::new("mdbook")
        .args(&["serve", "--port", &port.to_string(), "--open"])
        .status()
        .expect("Failed to execute mdbook serve");
    
    if !status.success() {
        eprintln!("❌ mdbook serve failed");
        std::process::exit(1);
    }
}

fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn find_available_port(start_port: u16) -> u16 {
    for port in start_port..=65535 {
        if !port_is_in_use(port) {
            return port;
        }
        if port != start_port {
            println!("⚠️  Port {} is in use, trying next port...", port - 1);
        }
    }
    
    eprintln!("❌ No available ports found");
    std::process::exit(1);
}

fn port_is_in_use(port: u16) -> bool {
    Command::new("lsof")
        .args(&["-i", &format!(":{}", port)])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
