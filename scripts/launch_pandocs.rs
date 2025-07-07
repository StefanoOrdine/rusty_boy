use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs;
use std::io;

fn main() {
    println!("🚀 Launching Pan Docs book...");
    
    // Get the project root directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let pandocs_dir = current_dir.join("resources").join("pandocs");
    
    println!("📁 Project root: {}", current_dir.display());
    println!("📚 Pandocs directory: {}", pandocs_dir.display());
    
    // Check if pandocs directory exists
    if !pandocs_dir.exists() {
        eprintln!("❌ Error: Pandocs directory not found at {}", pandocs_dir.display());
        std::process::exit(1);
    }
    
    // Change to pandocs directory
    env::set_current_dir(&pandocs_dir).expect("Failed to change to pandocs directory");
    
    // Setup Python virtual environment
    setup_python_env().expect("Failed to setup Python environment");
    
    // Check if mdbook is available
    if !command_exists("mdbook") {
        eprintln!("❌ Error: mdbook is not installed");
        eprintln!("Please install mdbook: cargo install mdbook");
        std::process::exit(1);
    }
    
    // Check if cargo is available
    if !command_exists("cargo") {
        eprintln!("❌ Error: cargo is not installed");
        eprintln!("Please install Rust and Cargo");
        std::process::exit(1);
    }
    
    // Build Rust preprocessors
    println!("🔧 Building Rust preprocessors...");
    let build_status = Command::new("cargo")
        .args(&["build", "--release", "--locked"])
        .status()
        .expect("Failed to execute cargo build");
    
    if !build_status.success() {
        eprintln!("❌ Failed to build Rust preprocessors");
        std::process::exit(1);
    }
    
    // Find an available port (starting from 3000)
    let port = find_available_port(3000);
    
    println!("🌐 Starting mdbook server on port {}...", port);
    println!("📖 The book will be available at: http://localhost:{}", port);
    println!("🔄 The server will watch for file changes and auto-reload");
    println!();
    println!("Press Ctrl+C to stop the server");
    println!();
    
    // Start the mdbook server
    let mut cmd = Command::new("mdbook");
    cmd.args(&["serve", "--port", &port.to_string(), "--open"]);
    
    // Set up environment for Python virtual environment
    let venv_path = pandocs_dir.join("env").join("bin");
    if let Ok(current_path) = env::var("PATH") {
        let new_path = format!("{}:{}", venv_path.display(), current_path);
        cmd.env("PATH", new_path);
    }
    cmd.env("VIRTUAL_ENV", pandocs_dir.join("env"));
    
    let status = cmd.status().expect("Failed to execute mdbook serve");
    
    if !status.success() {
        eprintln!("❌ mdbook serve failed");
        std::process::exit(1);
    }
}

fn setup_python_env() -> io::Result<()> {
    let venv_dir = Path::new("env");
    
    // Create virtual environment if it doesn't exist
    if !venv_dir.exists() {
        println!("🐍 Creating Python virtual environment...");
        let status = Command::new("python3")
            .args(&["-m", "venv", "env"])
            .status()?;
        
        if !status.success() {
            eprintln!("❌ Failed to create Python virtual environment");
            std::process::exit(1);
        }
    }
    
    // Check if we need to install requirements
    let requirements_file = Path::new("requirements.txt");
    let install_marker = venv_dir.join(".requirements_installed");
    
    let needs_install = !install_marker.exists() || 
        (requirements_file.exists() && 
         requirements_file.metadata()?.modified()? > install_marker.metadata()?.modified()?);
    
    if needs_install {
        println!("📦 Installing Python dependencies...");
        
        // Activate virtual environment and install requirements
        let pip_path = venv_dir.join("bin").join("pip");
        let status = Command::new(pip_path)
            .args(&["install", "-r", "requirements.txt"])
            .status()?;
        
        if !status.success() {
            eprintln!("❌ Failed to install Python requirements");
            std::process::exit(1);
        }
        
        // Create marker file
        fs::write(install_marker, "")?;
    } else {
        println!("✅ Python dependencies already up to date");
    }
    
    Ok(())
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
