use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

const IGNORE_PATTERNS: &[&str] = &[
    "node_modules/",
    "__pycache__/",
    ".git/",
    "dist/",
    "build/",
    "target/",
    "vendor/",
    "bin/",
    "obj/",
    ".idea/",
    ".vscode/",
    ".zig-cache/",
    "zig-out/",
    ".coverage/",
    "coverage/",
    "tmp/",
    "temp/",
    ".cache/",
    "cache/",
    "logs/",
    ".venv/",
    "venv/",
    "env/",
];

const LIMIT: usize = 100;

pub struct LsTool;

#[async_trait]
impl Tool for LsTool {
    fn name(&self) -> &str {
        "ls"
    }

    fn description(&self) -> &str {
        "List directory contents with detailed information including file types, sizes, and modification times."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The absolute path to the directory to list (defaults to current working directory)"
                },
                "ignore": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "List of glob patterns to ignore"
                }
            },
            "required": []
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let search_path = args.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(&context.working_directory);
        let ignore: Vec<String> = args.get("ignore")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|i| i.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        let resolved_path = if PathBuf::from(search_path).is_absolute() {
            PathBuf::from(search_path)
        } else {
            PathBuf::from(&context.working_directory).join(search_path)
        };

        let mut ignore_globs: Vec<String> = IGNORE_PATTERNS
            .iter()
            .map(|p| format!("!{}*", p))
            .collect();
        for ig in &ignore {
            ignore_globs.push(format!("!{}", ig));
        }

        let files = match ripgrep_files(&resolved_path, &ignore_globs) {
            Ok(f) => f,
            Err(e) => return error_result_with_path(format!("Failed to list directory: {}", e), resolved_path.to_string_lossy().to_string()),
        };

        let mut dirs: HashMap<String, bool> = HashMap::new();
        let mut files_by_dir: HashMap<String, Vec<String>> = HashMap::new();

        for file in &files {
            let parent = PathBuf::from(file).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or(".".to_string());
            let parts: Vec<&str> = parent.split('/').collect();

            for i in 0..=parts.len() {
                let dir_path = if i == 0 {
                    ".".to_string()
                } else {
                    parts[..i].join("/")
                };
                dirs.insert(dir_path, true);
            }

            let file_name = PathBuf::from(file).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            files_by_dir.entry(parent).or_default().push(file_name);
        }

        let mut output = String::new();
        output.push_str(&format!("{}/\n", resolved_path.display()));
        output.push_str(&render_dir(".", &dirs, &files_by_dir, 0));

        let truncated = files.len() >= LIMIT;

        let mut metadata = HashMap::new();
        metadata.insert("count".to_string(), serde_json::json!(files.len()));
        metadata.insert("truncated".to_string(), serde_json::json!(truncated));

        ToolResult {
            title: resolved_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
            output,
            metadata,
            attachments: Vec::new(),
            truncated,
            success: Some(true),
        }
    }
}

fn render_dir(dir_path: &str, dirs: &HashMap<String, bool>, files_by_dir: &HashMap<String, Vec<String>>, depth: usize) -> String {
    let mut output = String::new();
    let indent = "  ".repeat(depth);

    if depth > 0 {
        output.push_str(&format!("{}{}/\n", indent, PathBuf::from(dir_path).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()));
    }

    let mut children: Vec<String> = dirs.keys()
        .filter(|d| {
            let parent = PathBuf::from(*d).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
            parent == dir_path && *d != dir_path
        })
        .cloned()
        .collect();
    children.sort();

    for child in &children {
        output.push_str(&render_dir(child, dirs, files_by_dir, depth + 1));
    }

    let mut files_in_dir = files_by_dir.get(dir_path).cloned().unwrap_or_default();
    files_in_dir.sort();

    let child_indent = "  ".repeat(depth + 1);
    for file in &files_in_dir {
        output.push_str(&format!("{}{}\n", child_indent, file));
    }

    output
}

fn ripgrep_files(cwd: &PathBuf, ignore_globs: &[String]) -> Result<Vec<String>, String> {
    let mut args: Vec<String> = vec!["--files".to_string(), "--hidden".to_string(), "--glob=!.git/*".to_string()];
    for g in ignore_globs {
        args.push(format!("--glob={}", g));
    }

    let output = Command::new("rg")
        .args(&args)
        .current_dir(cwd)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() && output.status.code() != Some(1) {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .take(LIMIT)
        .map(|l| l.to_string())
        .collect();

    Ok(lines)
}

fn error_result_with_path(message: impl Into<String>, path: String) -> ToolResult {
    let mut metadata = HashMap::new();
    metadata.insert("error".to_string(), serde_json::json!(true));
    metadata.insert("path".to_string(), serde_json::json!(path));
    ToolResult {
        title: "Error".to_string(),
        output: message.into(),
        metadata,
        attachments: Vec::new(),
        truncated: false,
        success: Some(false),
    }
}