use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::fs;

const MAX_LINE_LENGTH: usize = 2000;
const MAX_RESULTS: usize = 100;

pub struct GrepTool;

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for patterns in file contents using regular expressions. Returns file paths and line numbers with matches sorted by modification time."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The regex pattern to search for in file contents"
                },
                "path": {
                    "type": "string",
                    "description": "The directory to search in (defaults to current working directory)"
                },
                "include": {
                    "type": "string",
                    "description": "File pattern to include in the search (e.g., \"*.js\", \"*.{ts,tsx}\")"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let pattern = args.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let search_path = args.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(&context.working_directory);
        let include = args.get("include")
            .and_then(|v| v.as_str());

        if pattern.is_empty() {
            return error_result("pattern is required");
        }

        let resolved_path = if PathBuf::from(search_path).is_absolute() {
            PathBuf::from(search_path)
        } else {
            PathBuf::from(&context.working_directory).join(search_path)
        };

        let matches = match ripgrep_grep(&resolved_path, pattern, include) {
            Ok(m) => m,
            Err(e) => return error_result_with_meta(format!("Grep search failed: {}", e), pattern.to_string(), resolved_path.to_string_lossy().to_string()),
        };

        let matches_count = matches.len();
        let mut matches_with_mtime: Vec<GrepMatch> = Vec::new();
        for m in matches {
            let full_path = if PathBuf::from(&m.path).is_absolute() {
                PathBuf::from(&m.path)
            } else {
                resolved_path.join(&m.path)
            };
            let mtime = fs::metadata(&full_path)
                .map(|meta| meta.modified().map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64).unwrap_or(0))
                .unwrap_or(0);
            matches_with_mtime.push(GrepMatch { path: m.path, line: m.line, text: m.text, mtime });
        }

        matches_with_mtime.sort_by(|a, b| b.mtime.cmp(&a.mtime));

        let truncated = matches_with_mtime.len() > MAX_RESULTS;
        let result_matches: Vec<GrepMatch> = matches_with_mtime.iter().take(MAX_RESULTS).cloned().collect();

        let mut output_lines: Vec<String> = Vec::new();
        output_lines.push(format!("Found {} matches{}", matches_count, if truncated { format!(" (showing first {})", MAX_RESULTS) } else { "".to_string() }));

        let mut current_file = "";
        for match_item in &result_matches {
            if current_file != match_item.path {
                if !current_file.is_empty() {
                    output_lines.push("".to_string());
                }
                current_file = &match_item.path;
                output_lines.push(format!("{}:", match_item.path));
            }
            let truncated_text = if match_item.text.len() > MAX_LINE_LENGTH {
                format!("{}...", &match_item.text[..MAX_LINE_LENGTH])
            } else {
                match_item.text.clone()
            };
            output_lines.push(format!("  Line {}: {}", match_item.line, truncated_text));
        }

        if truncated {
            output_lines.push("".to_string());
            output_lines.push(format!("(Results truncated: showing {} of {} matches. Use a more specific path or pattern.)", MAX_RESULTS, matches_count));
        }

        let mut metadata = HashMap::new();
        metadata.insert("pattern".to_string(), serde_json::json!(pattern));
        metadata.insert("path".to_string(), serde_json::json!(resolved_path.to_string_lossy().to_string()));
        if let Some(inc) = include {
            metadata.insert("include".to_string(), serde_json::json!(inc));
        }
        metadata.insert("matches".to_string(), serde_json::json!(matches_count));
        metadata.insert("truncated".to_string(), serde_json::json!(truncated));

        ToolResult {
            title: format!("Grep: {}", pattern),
            output: output_lines.join("\n"),
            metadata,
            attachments: Vec::new(),
            truncated,
            success: Some(true),
        }
    }
}

#[derive(Clone)]
struct GrepMatch {
    path: String,
    line: usize,
    text: String,
    mtime: i64,
}

struct RawMatch {
    path: String,
    line: usize,
    text: String,
}

fn ripgrep_grep(cwd: &PathBuf, pattern: &str, include: Option<&str>) -> Result<Vec<RawMatch>, String> {
    let mut args: Vec<String> = vec![
        "-nH".to_string(),
        "--hidden".to_string(),
        "--no-messages".to_string(),
        "--field-match-separator=|".to_string(),
        "--regexp".to_string(),
        pattern.to_string(),
    ];

    if let Some(inc) = include {
        args.push("--glob".to_string());
        args.push(inc.to_string());
    }

    args.push(cwd.to_string_lossy().to_string());

    let output = Command::new("rg")
        .args(&args)
        .current_dir(cwd)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() && output.status.code() != Some(1) {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let mut results: Vec<RawMatch> = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() >= 3 {
            let line_num = parts[1].parse::<usize>().unwrap_or(0);
            results.push(RawMatch {
                path: parts[0].to_string(),
                line: line_num,
                text: parts[2].to_string(),
            });
        }
    }

    Ok(results)
}

fn error_result(message: impl Into<String>) -> ToolResult {
    ToolResult {
        title: "Error".to_string(),
        output: message.into(),
        metadata: HashMap::new(),
        attachments: Vec::new(),
        truncated: false,
        success: Some(false),
    }
}

fn error_result_with_meta(message: impl Into<String>, pattern: String, path: String) -> ToolResult {
    let mut metadata = HashMap::new();
    metadata.insert("error".to_string(), serde_json::json!(true));
    metadata.insert("pattern".to_string(), serde_json::json!(pattern));
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