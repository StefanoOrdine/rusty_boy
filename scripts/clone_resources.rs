use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    let repositories = [
        ("https://github.com/rylev/DMG-01.git", "DMG-01"),
        ("https://github.com/Gekkio/mooneye-gb.git", "mooneye-gb"),
        ("https://github.com/Gekkio/gb-ctr.git", "gb-ctr"),
        ("https://github.com/Gekkio/mooneye-test-suite.git", "mooneye-test-suite"),
        ("https://github.com/gbdev/pandocs.git", "pandocs"),
    ];

    // Create resources directory if it doesn't exist
    let resources_dir = Path::new("resources");
    if !resources_dir.exists() {
        fs::create_dir_all(resources_dir).expect("Failed to create resources directory");
    }

    for (repo_url, folder_name) in repositories.iter() {
        let target_path = resources_dir.join(folder_name);
        
        if target_path.exists() {
            println!("Directory {} already exists, skipping clone", folder_name);
            continue;
        }

        println!("Cloning {} into resources/{}", repo_url, folder_name);
        
        let output = Command::new("git")
            .args(&["clone", repo_url, &target_path.to_string_lossy()])
            .output()
            .expect("Failed to execute git clone command");

        if output.status.success() {
            println!("Successfully cloned {}", folder_name);
        } else {
            eprintln!("Failed to clone {}: {}", folder_name, String::from_utf8_lossy(&output.stderr));
        }
    }

    println!("Resource cloning complete!");
}
