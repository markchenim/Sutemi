use serde::Serialize;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DashboardData {
    repo_path: String,
    repo_root: Option<String>,
    jj_version: Option<String>,
    status_summary: String,
    status: String,
    log: String,
    bookmarks: String,
    suggestions: Vec<String>,
    is_repo: bool,
    jj_available: bool,
    error: Option<String>,
}

#[derive(Debug)]
struct CommandOutput {
    stdout: String,
    stderr: String,
    success: bool,
}

impl CommandOutput {
    fn message(&self) -> String {
        if self.stdout.trim().is_empty() {
            self.stderr.trim().to_string()
        } else {
            self.stdout.trim().to_string()
        }
    }
}

fn normalized_path(repo_path: Option<String>) -> PathBuf {
    match repo_path {
        Some(path) if !path.trim().is_empty() => PathBuf::from(path),
        _ => env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn run_jj(current_dir: &Path, args: &[&str]) -> CommandOutput {
    match Command::new("jj").args(args).current_dir(current_dir).output() {
        Ok(output) => CommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            success: output.status.success(),
        },
        Err(error) => CommandOutput {
            stdout: String::new(),
            stderr: error.to_string(),
            success: false,
        },
    }
}

fn collect_suggestions(current_dir: &Path) -> Vec<String> {
    let mut suggestions = Vec::new();

    if let Some(parent) = current_dir.parent() {
        suggestions.push(parent.display().to_string());
    }

    suggestions.push(current_dir.display().to_string());

    if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
        suggestions.push(home);
    }

    suggestions.sort();
    suggestions.dedup();
    suggestions
}

fn summarize_status(output: &str) -> String {
    let normalized = output.to_ascii_lowercase();

    if normalized.contains("working copy is clean") || normalized.contains("nothing changed") {
        "Clean working copy".into()
    } else if normalized.trim().is_empty() {
        "No status output".into()
    } else {
        let changed_lines = output
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                trimmed.starts_with('A')
                    || trimmed.starts_with('M')
                    || trimmed.starts_with('D')
                    || trimmed.starts_with('R')
                    || trimmed.starts_with('C')
            })
            .count();

        if changed_lines == 0 {
            "Repository has activity".into()
        } else if changed_lines == 1 {
            "1 changed path".into()
        } else {
            format!("{changed_lines} changed paths")
        }
    }
}

#[tauri::command]
fn load_dashboard(repo_path: Option<String>) -> DashboardData {
    let current_dir = normalized_path(repo_path);
    let repo_path = current_dir.display().to_string();

    if !current_dir.exists() {
        return DashboardData {
            repo_path,
            repo_root: None,
            jj_version: None,
            status_summary: "Path not found".into(),
            status: "The selected path does not exist.".into(),
            log: "Update the repository path and refresh again.".into(),
            bookmarks: "Bookmarks appear after a valid repository is loaded.".into(),
            suggestions: collect_suggestions(&env::current_dir().unwrap_or_else(|_| PathBuf::from("."))),
            is_repo: false,
            jj_available: false,
            error: Some("Path not found".into()),
        };
    }

    let jj_version_output = run_jj(&current_dir, &["--version"]);
    let jj_available = jj_version_output.success;

    if !jj_available {
        return DashboardData {
            repo_path,
            repo_root: None,
            jj_version: None,
            status_summary: "jj unavailable".into(),
            status: "jj is not available on PATH.".into(),
            log: "Install Jujutsu and reopen Sutemi.".into(),
            bookmarks: "Bookmarks will appear here once jj is available.".into(),
            suggestions: collect_suggestions(&current_dir),
            is_repo: false,
            jj_available: false,
            error: Some(jj_version_output.message()),
        };
    }

    let repo_root_output = run_jj(&current_dir, &["root"]);
    let is_repo = repo_root_output.success;
    let repo_root = if is_repo {
        Some(repo_root_output.stdout.trim().to_string())
    } else {
        None
    };

    if !is_repo {
        return DashboardData {
            repo_path,
            repo_root,
            jj_version: Some(jj_version_output.stdout.trim().to_string()),
            status_summary: "Not a jj repository".into(),
            status: "No Jujutsu repository detected at this path.".into(),
            log: "Point the app at a repo to load history.".into(),
            bookmarks: "Point the app at a repo to load bookmarks.".into(),
            suggestions: collect_suggestions(&current_dir),
            is_repo: false,
            jj_available: true,
            error: Some(repo_root_output.message()),
        };
    }

    let status_output = run_jj(&current_dir, &["status"]);
    let log_output = run_jj(
        &current_dir,
        &[
            "log",
            "-n",
            "12",
            "--no-graph",
            "-T",
            "change_id.shortest(8) ++ \"  \" ++ description.first_line() ++ \"\\n\"",
        ],
    );
    let bookmarks_output = run_jj(&current_dir, &["bookmark", "list"]);
    let status_message = status_output.message();
    let mut error_messages = Vec::new();

    if !status_output.success {
        error_messages.push(format!("status: {}", status_output.message()));
    }
    if !log_output.success {
        error_messages.push(format!("log: {}", log_output.message()));
    }
    if !bookmarks_output.success {
        error_messages.push(format!("bookmarks: {}", bookmarks_output.message()));
    }

    DashboardData {
        repo_path,
        repo_root,
        jj_version: Some(jj_version_output.stdout.trim().to_string()),
        status_summary: summarize_status(&status_message),
        status: status_message,
        log: log_output.message(),
        bookmarks: bookmarks_output.message(),
        suggestions: collect_suggestions(&current_dir),
        is_repo: true,
        jj_available: true,
        error: if error_messages.is_empty() {
            None
        } else {
            Some(error_messages.join(" | "))
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_dashboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
