use chrono::{Local, Timelike};
use std::process::Command;

fn main() {
    let now = Local::now();
    let hour = now.hour();
    let minute = now.minute();

    let text_to_speak = format!("The time is {}: {}", hour, minute);

    match std::env::consts::OS {
        "linux" => {
            // linux-specific code
            let _ = Command::new("espeak")
                .arg(text_to_speak)
                .spawn()
                .expect("Failed to execute tts command");
        }
        "mac" => {
            // macOS-specific code
            let _ = Command::new("say")
                .arg(text_to_speak)
                .spawn()
                .expect("Failed to execute tts command");
        }
        "windows" => {
            // macOS-specific code
            let _ = Command::new("powershell")
                .arg("-Command")
                .arg(format!(
                    "[System.Speech.Synthesis.SpeechSynthesizer]::new().Speak('{}')",
                    text_to_speak
                ))
                .spawn()
                .expect("Failed to execute tts command");
        }
        _ => {
            // code for unsupported Operating Systems
            println!("Unsupported operating system");
        }
    }
}
