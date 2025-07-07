use std::env;
use std::fs;
use std::process::Command;

const BOOKMARK_FILE: &str = ".rust_docs_bookmark";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "save" => {
                if args.len() < 3 {
                    eprintln!("❌ Usage: cargo run --bin rust-docs save <page_url>");
                    eprintln!("Example: cargo run --bin rust-docs save \"book/ch01-01-installation.html\"");
                    std::process::exit(1);
                }
                save_bookmark(&args[2]);
            }
            "list" => {
                list_common_pages();
            }
            "help" | "--help" | "-h" => {
                show_help();
            }
            page => {
                // Treat as a direct page to open
                save_bookmark(page);
                open_rust_docs(Some(page));
            }
        }
    } else {
        // No arguments - open last bookmarked page or start from beginning
        let bookmark = load_bookmark();
        open_rust_docs(bookmark.as_deref());
    }
}

fn save_bookmark(page: &str) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bookmark_path = current_dir.join(BOOKMARK_FILE);
    
    if let Err(e) = fs::write(&bookmark_path, page) {
        eprintln!("⚠️  Warning: Failed to save bookmark: {}", e);
    } else {
        println!("📖 Bookmarked: {}", page);
    }
}

fn load_bookmark() -> Option<String> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bookmark_path = current_dir.join(BOOKMARK_FILE);
    
    if bookmark_path.exists() {
        match fs::read_to_string(&bookmark_path) {
            Ok(content) => {
                let bookmark = content.trim();
                if !bookmark.is_empty() {
                    println!("📚 Resuming from bookmark: {}", bookmark);
                    return Some(bookmark.to_string());
                }
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to read bookmark: {}", e);
            }
        }
    }
    
    None
}

fn open_rust_docs(page: Option<&str>) {
    println!("🦀 Opening Rust documentation...");
    
    // Get the Rust documentation path
    let doc_output = Command::new("rustup")
        .args(&["doc", "--path"])
        .output();
    
    let doc_path = match doc_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => {
            eprintln!("❌ Failed to get Rust documentation path");
            eprintln!("Make sure Rust is installed and rustup is available");
            std::process::exit(1);
        }
    };
    
    // Construct the full URL
    let url = if let Some(page) = page {
        if page.starts_with("http") || page.starts_with("file://") {
            // Already a full URL, use it directly
            page.to_string()
        } else {
            // Relative path, construct URL from doc_path
            // Remove the index.html from doc_path to get the base directory
            let base_path = if doc_path.ends_with("index.html") {
                doc_path.trim_end_matches("index.html")
            } else {
                &doc_path
            };
            format!("file://{}{}", base_path, page.trim_start_matches('/'))
        }
    } else {
        format!("file://{}", doc_path)
    };
    
    println!("🌐 Opening: {}", url);
    
    // Open in default browser
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(&url).status()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(&url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "start", &url]).status()
    } else {
        eprintln!("❌ Unsupported operating system");
        std::process::exit(1);
    };
    
    match result {
        Ok(status) if status.success() => {
            println!("✅ Documentation opened successfully!");
            println!();
            show_usage_tips();
        }
        _ => {
            eprintln!("❌ Failed to open documentation in browser");
            eprintln!("You can manually open: {}", url);
        }
    }
}

fn list_common_pages() {
    println!("📚 Common Rust documentation pages:");
    println!();
    
    let pages = vec![
        ("book/", "The Rust Programming Language (Book)"),
        ("book/ch01-00-getting-started.html", "Getting Started"),
        ("book/ch02-00-guessing-game-tutorial.html", "Guessing Game Tutorial"),
        ("book/ch03-00-common-programming-concepts.html", "Common Programming Concepts"),
        ("book/ch04-00-understanding-ownership.html", "Understanding Ownership"),
        ("book/ch05-00-structs.html", "Using Structs"),
        ("book/ch06-00-enums.html", "Enums and Pattern Matching"),
        ("book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html", "Managing Growing Projects"),
        ("book/ch08-00-common-collections.html", "Common Collections"),
        ("book/ch09-00-error-handling.html", "Error Handling"),
        ("book/ch10-00-generics.html", "Generic Types, Traits, and Lifetimes"),
        ("std/", "Standard Library Documentation"),
        ("reference/", "The Rust Reference"),
        ("nomicon/", "The Rustonomicon (Unsafe Rust)"),
        ("edition-guide/", "Edition Guide"),
    ];
    
    for (i, (path, description)) in pages.iter().enumerate() {
        println!("  {}. {} - {}", i + 1, description, path);
    }
    
    println!();
    println!("💡 Usage examples:");
    println!("  cargo run --bin rust-docs book/ch04-00-understanding-ownership.html");
    println!("  cargo run --bin rust-docs save \"book/ch05-01-defining-structs.html\"");
}

fn show_help() {
    println!("🦀 Rust Documentation Launcher with Bookmarking");
    println!();
    println!("USAGE:");
    println!("  cargo run --bin rust-docs [COMMAND] [PAGE]");
    println!();
    println!("COMMANDS:");
    println!("  (no args)           Open documentation from last bookmark or start page");
    println!("  <page>              Open specific page and bookmark it");
    println!("  save <page>         Save a bookmark without opening");
    println!("  list                List common documentation pages");
    println!("  help                Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  cargo run --bin rust-docs");
    println!("  cargo run --bin rust-docs book/ch04-00-understanding-ownership.html");
    println!("  cargo run --bin rust-docs save \"std/vec/struct.Vec.html\"");
    println!("  cargo run --bin rust-docs list");
    println!();
    println!("📁 Bookmark file: {}", BOOKMARK_FILE);
    println!("   This file will be created in your project root and can be committed to git.");
}

fn show_usage_tips() {
    println!("💡 Usage tips:");
    println!("  • When you find an interesting page, copy its path from the URL");
    println!("  • Save it with: cargo run --bin rust-docs save \"<page-path>\"");
    println!("  • Next time, just run: cargo run --bin rust-docs");
    println!("  • The bookmark file ({}) can be committed to git", BOOKMARK_FILE);
    println!("  • Use 'cargo run --bin rust-docs list' to see common pages");
}
