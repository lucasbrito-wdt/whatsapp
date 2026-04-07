use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
}

fn parse_pactl_output(output: &str) -> Vec<AudioDevice> {
    output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return None;
            }
            let id = parts[0].to_string();
            let name = parts[1].to_string();
            Some(AudioDevice { id, name })
        })
        .collect()
}

fn run_pactl(args: &[&str]) -> Result<Vec<AudioDevice>, String> {
    let output = Command::new("pactl")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run pactl: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "pactl exited with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    Ok(parse_pactl_output(&stdout))
}

#[tauri::command]
pub async fn list_audio_inputs() -> Result<Vec<AudioDevice>, String> {
    run_pactl(&["list", "sources", "short"])
}

#[tauri::command]
pub async fn list_audio_outputs() -> Result<Vec<AudioDevice>, String> {
    run_pactl(&["list", "sinks", "short"])
}
