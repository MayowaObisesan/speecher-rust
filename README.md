# Rust Text-to-Speech Application

This Rust application announces the current time using text-to-speech synthesis. It utilizes system commands to perform
text-to-speech on different operating systems.

## Requirements

- Rust programming language installed on your system
- For Linux: `espeak` command-line tool installed
- For macOS: `say` command available
- For Windows: PowerShell available

## Usage

1. Clone or download the repository.

2. Compile the Rust code using Cargo:

3. Run the executable:

4. If you want a release version run: `cargo build --release`

Replace `your_executable_name` with the actual name of the compiled Rust executable.

## Cron Job (Optional)

If you want to run the application in the background at regular intervals using a cron job, you can add the following
line to your crontab file (`crontab -e`):
Then add this command:
`*/30 * * * * /path/to/your_executable_name`

Replace `/path/to/your_executable_name` with the actual path to your compiled Rust executable.

## Notes

- Adjust the text-to-speech command based on your operating system in the Rust code (`main.rs`).
- Ensure that the necessary text-to-speech tools are installed and available in your system's PATH.
- This application is designed to run indefinitely and will announce the current time at regular intervals.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
