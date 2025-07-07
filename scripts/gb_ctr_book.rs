use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const BOOKMARK_FILE: &str = ".gb_ctr_bookmark";
const GB_CTR_DIR: &str = "resources/gb-ctr";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "build" => {
                build_book();
            }
            "open" => {
                open_book();
            }
            "save" => {
                if args.len() < 3 {
                    eprintln!("❌ Usage: cargo run --bin gb-ctr-book save <page_number>");
                    eprintln!("Example: cargo run --bin gb-ctr-book save 25");
                    std::process::exit(1);
                }
                if let Ok(page) = args[2].parse::<u32>() {
                    save_bookmark(page);
                } else {
                    eprintln!("❌ Invalid page number: {}", args[2]);
                    std::process::exit(1);
                }
            }
            "clean" => {
                clean_build();
            }
            "help" | "--help" | "-h" => {
                show_help();
            }
            _ => {
                eprintln!("❌ Unknown command: {}", args[1]);
                show_help();
                std::process::exit(1);
            }
        }
    } else {
        // No arguments - open existing book, resuming from bookmark if available
        let bookmark = load_bookmark();
        if let Some(page) = bookmark {
            println!("📖 Last bookmarked page: {}", page);
        }
        open_book();
    }
}

fn get_gb_ctr_path() -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let gb_ctr_path = current_dir.join(GB_CTR_DIR);
    
    if !gb_ctr_path.exists() {
        eprintln!("❌ GB-CTR directory not found at: {}", gb_ctr_path.display());
        eprintln!("Make sure you're running this from the project root.");
        std::process::exit(1);
    }
    
    gb_ctr_path.to_string_lossy().to_string()
}

fn build_book() {
    println!("🔨 Building Game Boy Complete Technical Reference...");
    let gb_ctr_path = get_gb_ctr_path();
    
    let result = Command::new("just")
        .arg("build")
        .current_dir(&gb_ctr_path)
        .status();
    
    match result {
        Ok(status) if status.success() => {
            println!("✅ Book built successfully!");
        }
        Ok(_) => {
            eprintln!("❌ Failed to build book");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ Failed to run just command: {}", e);
            eprintln!("Make sure 'just' is installed and available in PATH");
            eprintln!("You can install it with: brew install just");
            std::process::exit(1);
        }
    }
}

fn open_book() {
    let gb_ctr_path = get_gb_ctr_path();
    let pdf_path = Path::new(&gb_ctr_path).join("gbctr.pdf");
    
    if !pdf_path.exists() {
        eprintln!("❌ PDF not found at: {}", pdf_path.display());
        eprintln!("Run 'cargo run --bin gb-ctr-book build' first to build the book.");
        std::process::exit(1);
    }
    
    open_pdf(&pdf_path.to_string_lossy());
}

fn open_pdf(pdf_path: &str) {
    println!("📚 Opening Game Boy Complete Technical Reference in browser...");
    
    let file_url = format!("file://{}", pdf_path);
    
    let result = if cfg!(target_os = "macos") {
        // On macOS, use Google Chrome to open the PDF
        Command::new("open")
            .arg("-a")
            .arg("Google Chrome")
            .arg(&file_url)
            .status()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(&file_url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "start", &file_url]).status()
    } else {
        eprintln!("❌ Unsupported operating system");
        std::process::exit(1);
    };
    
    match result {
        Ok(status) if status.success() => {
            println!("✅ Book opened successfully in browser!");
            show_usage_tips();
        }
        _ => {
            eprintln!("❌ Failed to open PDF in browser");
            eprintln!("You can manually open: {}", file_url);
        }
    }
}

fn save_bookmark(page: u32) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bookmark_path = current_dir.join(BOOKMARK_FILE);
    
    if let Err(e) = fs::write(&bookmark_path, page.to_string()) {
        eprintln!("⚠️  Warning: Failed to save bookmark: {}", e);
    } else {
        println!("📖 Bookmarked page: {}", page);
    }
}

fn load_bookmark() -> Option<u32> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bookmark_path = current_dir.join(BOOKMARK_FILE);
    
    if bookmark_path.exists() {
        match fs::read_to_string(&bookmark_path) {
            Ok(content) => {
                let content = content.trim();
                if let Ok(page) = content.parse::<u32>() {
                    println!("📚 Resuming from bookmarked page: {}", page);
                    return Some(page);
                }
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to read bookmark: {}", e);
            }
        }
    }
    
    None
}

fn clean_build() {
    println!("🧹 Cleaning build artifacts...");
    let gb_ctr_path = get_gb_ctr_path();
    
    // Remove the PDF file
    let pdf_path = Path::new(&gb_ctr_path).join("gbctr.pdf");
    if pdf_path.exists() {
        if let Err(e) = fs::remove_file(&pdf_path) {
            eprintln!("⚠️  Warning: Failed to remove PDF: {}", e);
        } else {
            println!("🗑️  Removed: gbctr.pdf");
        }
    }
    
    // Remove config.json if it exists
    let config_path = Path::new(&gb_ctr_path).join("config.json");
    if config_path.exists() {
        if let Err(e) = fs::remove_file(&config_path) {
            eprintln!("⚠️  Warning: Failed to remove config.json: {}", e);
        } else {
            println!("🗑️  Removed: config.json");
        }
    }
    
    println!("✅ Clean completed!");
}

fn show_help() {
    println!("📚 Game Boy Complete Technical Reference Launcher");
    println!();
    println!("USAGE:");
    println!("  cargo run --bin gb-ctr-book [COMMAND] [ARGS]");
    println!();
    println!("COMMANDS:");
    println!("  (no args)           Open existing book from last bookmark");
    println!("  build               Build the book (PDF)");
    println!("  open                Open the existing PDF in Google Chrome");
    println!("  save <number>       Save a page bookmark without opening");
    println!("  clean               Remove build artifacts");
    println!("  help                Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  cargo run --bin gb-ctr-book");
    println!("  cargo run --bin gb-ctr-book build");
    println!("  cargo run --bin gb-ctr-book open");
    println!("  cargo run --bin gb-ctr-book save 42");
    println!();
    println!("REQUIREMENTS:");
    println!("  • just (install with: brew install just)");
    println!("  • typst (install with: brew install typst)");
    println!();
    println!("📁 Bookmark file: {}", BOOKMARK_FILE);
    println!("📂 Book directory: {}", GB_CTR_DIR);
}

fn show_usage_tips() {
    println!();
    println!("💡 Usage tips:");
    println!("  • Use 'cargo run --bin gb-ctr-book save <N>' to bookmark page N");
    println!("  • In Chrome PDF viewer, press Ctrl+G to 'Go to page' for quick navigation");
    println!("  • The bookmark file ({}) can be committed to git", BOOKMARK_FILE);
    println!("  • Run 'cargo run --bin gb-ctr-book clean' to remove build files");
}
