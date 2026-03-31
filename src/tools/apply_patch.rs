use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

const BEGIN_PATCH_MARKER: &str = "*** Begin Patch";
const END_PATCH_MARKER: &str = "*** End Patch";
const ADD_FILE_MARKER: &str = "*** Add File: ";
const DELETE_FILE_MARKER: &str = "*** Delete File: ";
const UPDATE_FILE_MARKER: &str = "*** Update File: ";
const MOVE_TO_MARKER: &str = "*** Move to: ";
const EOF_MARKER: &str = "*** End of File";

pub struct ApplyPatchTool;

#[async_trait]
impl Tool for ApplyPatchTool {
    fn name(&self) -> &str {
        "apply_patch"
    }

    fn description(&self) -> &str {
        "Apply a patch to one or more files using the apply_patch format."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "Patch content using the *** Begin Patch/End Patch format."
                }
            },
            "required": ["input"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let input = args.get("input")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if input.trim().is_empty() {
            return error_result("Patch input is required");
        }

        let hunks = match parse_patch(input) {
            Ok(h) => h,
            Err(e) => return error_result(format!("Failed to parse patch: {}", e)),
        };

        if hunks.is_empty() {
            return error_result("No file operations found in patch");
        }

        let workdir = PathBuf::from(&context.working_directory);
        let mut added: Vec<String> = Vec::new();
        let mut modified: Vec<String> = Vec::new();
        let mut deleted: Vec<String> = Vec::new();

        for hunk in &hunks {
            match apply_hunk(hunk, &workdir) {
                Ok(path) => match hunk.kind {
                    HunkKind::Add => added.push(path),
                    HunkKind::Update => modified.push(path),
                    HunkKind::Delete => deleted.push(path),
                },
                Err(e) => return error_result(format!("Failed to apply patch: {}", e)),
            }
        }

        let output = format!(
            "Added: {}\nModified: {}\nDeleted: {}",
            added.join(", "),
            modified.join(", "),
            deleted.join(", ")
        );

        ToolResult {
            title: "Patch Applied".to_string(),
            output,
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }
}

#[derive(Debug, Clone)]
enum HunkKind {
    Add,
    Delete,
    Update,
}

#[derive(Debug, Clone)]
struct Hunk {
    kind: HunkKind,
    path: String,
    contents: Option<String>,
    chunks: Vec<UpdateChunk>,
    move_path: Option<String>,
}

#[derive(Debug, Clone)]
struct UpdateChunk {
    old_lines: Vec<String>,
    new_lines: Vec<String>,
    is_end_of_file: bool,
}

fn parse_patch(input: &str) -> Result<Vec<Hunk>, String> {
    let lines: Vec<&str> = input.trim().lines().collect();
    
    if lines.first().map(|l| l.trim()) != Some(BEGIN_PATCH_MARKER) {
        return Err("The first line of the patch must be '*** Begin Patch'".to_string());
    }
    if lines.last().map(|l| l.trim()) != Some(END_PATCH_MARKER) {
        return Err("The last line of the patch must be '*** End Patch'".to_string());
    }

    let mut hunks: Vec<Hunk> = Vec::new();
    let mut i = 1;

    while i < lines.len() - 1 {
        let line = lines[i].trim();

        if line.starts_with(ADD_FILE_MARKER) {
            let path = line[ADD_FILE_MARKER.len()..].to_string();
            let mut contents = String::new();
            i += 1;

            while i < lines.len() - 1 && lines[i].starts_with('+') {
                contents.push_str(&lines[i][1..]);
                contents.push('\n');
                i += 1;
            }

            hunks.push(Hunk { kind: HunkKind::Add, path, contents: Some(contents), chunks: Vec::new(), move_path: None });
        } else if line.starts_with(DELETE_FILE_MARKER) {
            let path = line[DELETE_FILE_MARKER.len()..].to_string();
            hunks.push(Hunk { kind: HunkKind::Delete, path, contents: None, chunks: Vec::new(), move_path: None });
            i += 1;
        } else if line.starts_with(UPDATE_FILE_MARKER) {
            let path = line[UPDATE_FILE_MARKER.len()..].to_string();
            i += 1;

            let mut move_path: Option<String> = None;
            if lines[i].trim().starts_with(MOVE_TO_MARKER) {
                move_path = Some(lines[i].trim()[MOVE_TO_MARKER.len()..].to_string());
                i += 1;
            }

            let mut chunks: Vec<UpdateChunk> = Vec::new();
            while i < lines.len() - 1 && !lines[i].starts_with("***") {
                let chunk = parse_update_chunk(&lines, i);
                chunks.push(chunk.chunk);
                i += chunk.consumed;
            }

            if chunks.is_empty() {
                return Err(format!("Update file hunk for '{}' has no chunks", path));
            }

            hunks.push(Hunk { kind: HunkKind::Update, path, contents: None, chunks, move_path });
        } else {
            i += 1;
        }
    }

    Ok(hunks)
}

fn parse_update_chunk(lines: &[&str], start_idx: usize) -> UpdateChunkResult {
    let mut old_lines: Vec<String> = Vec::new();
    let mut new_lines: Vec<String> = Vec::new();
    let mut is_end_of_file = false;
    let mut consumed = 0;
    let mut i = start_idx;

    while i < lines.len() && !lines[i].starts_with("***") && !lines[i].trim().starts_with("@@") {
        let line = lines[i];

        if line == EOF_MARKER {
            is_end_of_file = true;
            consumed += 1;
            break;
        }

        if line.starts_with(' ') {
            let content = line[1..].to_string();
            old_lines.push(content.clone());
            new_lines.push(content);
        } else if line.starts_with('-') {
            old_lines.push(line[1..].to_string());
        } else if line.starts_with('+') {
            new_lines.push(line[1..].to_string());
        } else if line.is_empty() {
            old_lines.push(String::new());
            new_lines.push(String::new());
        } else {
            break;
        }

        i += 1;
        consumed += 1;
    }

    UpdateChunkResult {
        chunk: UpdateChunk { old_lines, new_lines, is_end_of_file },
        consumed,
    }
}

struct UpdateChunkResult {
    chunk: UpdateChunk,
    consumed: usize,
}

fn apply_hunk(hunk: &Hunk, workdir: &PathBuf) -> Result<String, String> {
    let resolved_path = workdir.join(&hunk.path);

    match hunk.kind {
        HunkKind::Add => {
            if let Some(ref contents) = hunk.contents {
                if let Some(parent) = resolved_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::write(&resolved_path, contents).map_err(|e| e.to_string())?;
            }
            Ok(hunk.path.clone())
        }
        HunkKind::Delete => {
            if resolved_path.exists() {
                fs::remove_file(&resolved_path).map_err(|e| e.to_string())?;
            }
            Ok(hunk.path.clone())
        }
        HunkKind::Update => {
            let old_content = fs::read_to_string(&resolved_path).map_err(|e| e.to_string())?;
            let new_content = apply_chunks(&old_content, &hunk.chunks)?;

            let target_path = if let Some(ref move_path) = hunk.move_path {
                let move_resolved = workdir.join(move_path);
                if let Some(parent) = move_resolved.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::write(&move_resolved, &new_content).map_err(|e| e.to_string())?;
                fs::remove_file(&resolved_path).map_err(|e| e.to_string())?;
                move_path.clone()
            } else {
                fs::write(&resolved_path, &new_content).map_err(|e| e.to_string())?;
                hunk.path.clone()
            };

            Ok(target_path)
        }
    }
}

fn apply_chunks(content: &str, chunks: &[UpdateChunk]) -> Result<String, String> {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut replacements: Vec<(usize, usize, Vec<String>)> = Vec::new();
    let mut line_index = 0;

    for chunk in chunks {
        if chunk.old_lines.is_empty() {
            replacements.push((lines.len(), 0, chunk.new_lines.clone()));
            continue;
        }

        let found = seek_sequence(&lines, &chunk.old_lines, line_index, chunk.is_end_of_file);
        if found.is_none() {
            return Err(format!("Failed to find expected lines:\n{}", chunk.old_lines.join("\n")));
        }

        let found = found.unwrap();
        replacements.push((found, chunk.old_lines.len(), chunk.new_lines.clone()));
        line_index = found + chunk.old_lines.len();
    }

    replacements.sort_by_key(|r| r.0);

    let mut result = lines.clone();
    for i in (0..replacements.len()).rev() {
        let (start_idx, old_len, new_segment) = &replacements[i];
        result.splice(*start_idx..(*start_idx + *old_len), new_segment.clone());
    }

    if result.last().map(|l| !l.is_empty()) == Some(true) {
        result.push(String::new());
    }

    Ok(result.join("\n"))
}

fn seek_sequence(lines: &[String], pattern: &[String], start_index: usize, eof: bool) -> Option<usize> {
    if pattern.is_empty() {
        return None;
    }

    if eof {
        let from_end = lines.len() - pattern.len();
        if from_end >= start_index {
            let matches = pattern.iter().enumerate()
                .all(|(j, p)| lines[from_end + j] == *p);
            if matches {
                return Some(from_end);
            }
        }
    }

    for i in start_index..=(lines.len() - pattern.len()) {
        let matches = pattern.iter().enumerate()
            .all(|(j, p)| lines[i + j] == *p);
        if matches {
            return Some(i);
        }
    }

    for i in start_index..=(lines.len() - pattern.len()) {
        let matches = pattern.iter().enumerate()
            .all(|(j, p)| lines[i + j].trim_end() == p.trim_end());
        if matches {
            return Some(i);
        }
    }

    None
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