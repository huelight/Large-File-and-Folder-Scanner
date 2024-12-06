# Large File and Folder Scanner

This Rust program scans your local filesystem for large files and folders based on customizable size thresholds. It helps you identify disk space hogs so you can decide what to delete or move.

## Features
- **Detect Large Files**: Identifies files larger than a specified size (default: 1 GB).
- **Analyze Large Folders**: Scans folder sizes recursively, reporting those above a defined threshold (default: 5 GB).
- **Customizable Settings**: Easily adjust size thresholds and starting paths in the script.
- **User-Friendly Output**: Displays file and folder sizes in readable units.

## Requirements
- **Rust**: Ensure you have the latest stable version of Rust installed.
- **Dependencies**: This script uses the following crates:
  - [`byte_unit`](https://crates.io/crates/byte_unit): For human-readable size formatting.
  - [`walkdir`](https://crates.io/crates/walkdir): For efficient directory traversal.

## Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd check_file
2. Build the project to install dependencies:
    ```bash
    cargo build

## Usage
1. Modify the script as needed:
   - Change the starting directory by editing `start_path`.
   - Update file and folder size thresholds by modifying `file_size_threshold` and `folder_size_threshold`.
2. Run the program:
    ```bash
    cargo run
3. View the output, which lists:
   - Large files with their paths and sizes.
   - Large folders with their paths and total sizes.

## Configuration
You can adjust the following settings in the script:
   - `start_path`: Defines where the scan begins (default: C:/).
  - `file_size_threshold`: Sets the minimum size for large files (default: 1 GB).
  - `folder_size_threshold`: Sets the minimum size for large folders (default: 5 GB).

## Example Output
    ```plaintext
    Scanning for large files and folders...
    Large file: C:/example/largefile.mp4 (2.3 GB)
    Large folder: C:/example/large_folder (7.8 GB)

## Contributing
Contributions are welcome! Please open issues for bugs or features and submit pull requests to enhance the tool.