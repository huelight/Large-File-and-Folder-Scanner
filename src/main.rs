use byte_unit::Byte;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let start_path = "C:/"; // Starting directory for the search
    let file_size_threshold = 1_073_741_824; // 1 GB in bytes
    let folder_size_threshold = 5_000_000_000; // 5 GB in bytes (you can adjust this)

    println!("Scanning for large files and folders...");

    // Track large files
    for entry in WalkDir::new(start_path)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path) {
                if metadata.len() > file_size_threshold {
                    println!(
                        "Large file: {} ({})",
                        path.display(),
                        Byte::from_bytes(metadata.len() as u128).get_appropriate_unit(false)
                    );
                }
            }
        }
    }

    let mut folder_sizes = Vec::new();

    for entry in WalkDir::new(start_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_dir() {
            let total_size = calculate_folder_size(path);
            if total_size > folder_size_threshold {
                folder_sizes.push((path.to_path_buf(), total_size));
            }
        }
    }

    for (folder, size) in folder_sizes {
        println!(
            "Large folder: {} ({})",
            folder.display(),
            Byte::from_bytes(size as u128).get_appropriate_unit(false)
        );
    }
}

// Function to calculate folder size recursively
fn calculate_folder_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|e| fs::metadata(e.path()).ok())
        .map(|metadata| metadata.len())
        .sum()
}
