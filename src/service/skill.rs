use crate::core::interfaces::storage::{Storage, SkillRegistry, Skill, CreateRegistryData, CreateSkillData};
use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;

pub struct SkillService {
    storage: Box<dyn Storage>,
    workspace: String,
}

impl SkillService {
    pub fn new(storage: Box<dyn Storage>, workspace: String) -> Self {
        Self { storage, workspace }
    }

    fn get_skills_dir(&self) -> PathBuf {
        PathBuf::from(&self.workspace).join(".openzerg").join("skills")
    }

    fn get_skill_path(&self, registry_name: &str, skill_name: &str) -> PathBuf {
        self.get_skills_dir().join(registry_name).join(skill_name)
    }

    pub async fn list_registries(&self) -> Result<Vec<SkillRegistry>> {
        self.storage.list_registries().await
    }

    pub async fn get_registry(&self, id: &str) -> Result<Option<SkillRegistry>> {
        self.storage.get_registry(id).await
    }

    pub async fn add_registry(&self, name: &str, url: &str, api_key: Option<&str>) -> Result<SkillRegistry> {
        let existing = self.storage.get_registry_by_name(name).await?;
        if existing.is_some() {
            return Err(anyhow!("Registry '{}' already exists", name));
        }

        let clean_url = url.trim_end_matches('/');
        let data = CreateRegistryData {
            name: name.to_string(),
            url: clean_url.to_string(),
            api_key: api_key.map(|s| s.to_string()),
        };

        self.storage.save_registry(data).await
    }

    pub async fn remove_registry(&self, id: &str) -> Result<()> {
        let registry = self.storage.get_registry(id).await?
            .ok_or_else(|| anyhow!("Registry not found: {}", id))?;

        let skills = self.storage.list_skills().await?;
        for skill in skills {
            if skill.registry_id == id {
                let skill_path = PathBuf::from(&skill.folder_path);
                if skill_path.exists() {
                    fs::remove_dir_all(&skill_path)?;
                }
            }
        }

        self.storage.delete_registry(id).await
    }

    pub async fn list_installed_skills(&self) -> Result<Vec<Skill>> {
        self.storage.list_skills().await
    }

    pub async fn get_skill(&self, full_name: &str, include_body: bool) -> Result<Option<(Skill, Option<String>, Vec<String>)>> {
        let skill = self.storage.get_skill(full_name).await?;
        if skill.is_none() {
            return Ok(None);
        }

        let skill = skill.unwrap();
        let skill_dir = PathBuf::from(&skill.folder_path);
        let mut body: Option<String> = None;
        let mut resources: Vec<String> = Vec::new();

        if skill_dir.exists() {
            for entry in walkdir::WalkDir::new(&skill_dir).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let relative = entry.path().strip_prefix(&skill_dir)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();

                    if relative == "SKILL.md" && include_body {
                        body = fs::read_to_string(entry.path()).ok();
                    } else if !relative.is_empty() {
                        resources.push(relative);
                    }
                }
            }
        }

        Ok(Some((skill, body, resources)))
    }

    pub async fn get_skills_for_context(&self) -> Result<Vec<Skill>> {
        self.storage.list_skills().await
    }
}

fn parse_skill_md(content: &str) -> Result<(String, String, String)> {
    let re = Regex::new(r"^---\n([\s\S]*?)\n---\n([\s\S]*)$").unwrap();
    let caps = re.captures(content)
        .ok_or_else(|| anyhow!("Invalid format: missing YAML frontmatter"))?;

    let yaml_content = &caps[1];
    let body = caps[2].trim();

    let frontmatter: serde_yaml::Value = serde_yaml::from_str(yaml_content)
        .map_err(|e| anyhow!("Invalid YAML: {}", e))?;

    let name = frontmatter.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing required field: name"))?;

    let description = frontmatter.get("description")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing required field: description"))?;

    Ok((name.to_string(), description.to_string(), body.to_string()))
}