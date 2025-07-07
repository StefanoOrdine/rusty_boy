use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "help" | "--help" | "-h" => {
                show_help();
                return;
            }
            _ => {
                eprintln!("❌ Unknown argument: {}", args[1]);
                show_help();
                std::process::exit(1);
            }
        }
    }
    
    println!("🚀 Launching Rusty Boy Development Environment");
    println!("===============================================");
    println!();
    
    // Step 1: Clone resources
    println!("📥 Step 1: Cloning resources...");
    run_command("clone-resources", "Cloning external resources");
    
    // Brief pause between operations
    thread::sleep(Duration::from_millis(500));
    
    // Step 2: Launch documentation
    println!();
    println!("📚 Step 2: Launching documentation...");
    
    // Launch Rust docs
    println!("  🦀 Opening Rust documentation...");
    run_command_background("rust-docs", "Rust documentation");
    
    // Brief pause
    thread::sleep(Duration::from_millis(1000));
    
    // Launch Pandocs
    println!("  📖 Opening Pandocs (Game Boy development guide)...");
    run_command_background("launch-pandocs", "Pandocs");
    
    // Brief pause
    thread::sleep(Duration::from_millis(1000));
    
    // Launch DMG-01 docs
    println!("  🎮 Opening DMG-01 documentation...");
    run_command_background("launch-dmg01", "DMG-01 docs");
    
    // Brief pause
    thread::sleep(Duration::from_millis(1000));
    
    // Launch GB-CTR book
    println!("  📕 Opening Game Boy Complete Technical Reference...");
    run_command_background("gb-ctr-book", "GB-CTR book");
    
    println!();
    println!("✅ Development environment launched successfully!");
    println!();
    show_summary();
}

fn run_command(binary_name: &str, description: &str) {
    let result = Command::new("cargo")
        .args(&["run", "--bin", binary_name])
        .status();
    
    match result {
        Ok(status) if status.success() => {
            println!("  ✅ {}", description);
        }
        Ok(_) => {
            println!("  ⚠️  {} completed with warnings", description);
        }
        Err(e) => {
            println!("  ❌ Failed to run {}: {}", description, e);
        }
    }
}

fn run_command_background(binary_name: &str, description: &str) {
    let result = Command::new("cargo")
        .args(&["run", "--bin", binary_name])
        .spawn();
    
    match result {
        Ok(_) => {
            println!("    ✅ {} launched", description);
        }
        Err(e) => {
            println!("    ❌ Failed to launch {}: {}", description, e);
        }
    }
}

fn show_help() {
    println!("🚀 Rusty Boy Development Environment Launcher");
    println!();
    println!("USAGE:");
    println!("  cargo run --bin launch-all-docs [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("  (no args)           Launch the complete development environment");
    println!("  help                Show this help message");
    println!();
    println!("WHAT IT DOES:");
    println!("  1. Clones external resources (mooneye-gb, pandocs, etc.)");
    println!("  2. Opens Rust documentation in browser");
    println!("  3. Opens Pandocs (Game Boy development guide)");
    println!("  4. Opens DMG-01 documentation");
    println!("  5. Opens Game Boy Complete Technical Reference");
    println!();
    println!("INDIVIDUAL COMMANDS:");
    println!("  cargo run --bin clone-resources    # Clone external resources");
    println!("  cargo run --bin rust-docs          # Open Rust documentation");
    println!("  cargo run --bin launch-pandocs     # Open Pandocs");
    println!("  cargo run --bin launch-dmg01       # Open DMG-01 docs");
    println!("  cargo run --bin gb-ctr-book        # Open GB-CTR book");
}

fn show_summary() {
    println!("💡 What's now available:");
    println!("  • External resources cloned to resources/ folder");
    println!("  • Rust documentation open in browser");
    println!("  • Pandocs (Game Boy dev guide) running locally");
    println!("  • DMG-01 documentation available");
    println!("  • Game Boy Complete Technical Reference open");
    println!();
    println!("🔧 Happy Game Boy development!");
    println!();
    println!("💭 Tip: You can run individual components with:");
    println!("     cargo run --bin <component-name>");
}
