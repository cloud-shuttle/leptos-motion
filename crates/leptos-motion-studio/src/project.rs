//! Project management for Motion Studio

use crate::{Result, StudioError, timeline::Timeline3D, transforms::Transform3D};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Studio project containing animations and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioProject {
    /// Project ID
    pub id: Uuid,
    /// Project name
    pub name: String,
    /// Project version
    pub version: String,
    /// Project description
    pub description: String,
    /// Project settings
    pub settings: ProjectSettings,
    /// Animations in the project
    pub animations: HashMap<Uuid, ProjectAnimation>,
    /// Assets (paths, images, etc.)
    pub assets: HashMap<Uuid, ProjectAsset>,
    /// Created timestamp
    pub created: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified: chrono::DateTime<chrono::Utc>,
}

impl StudioProject {
    /// Create new project
    pub fn new(name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: String::new(),
            settings: ProjectSettings::default(),
            animations: HashMap::new(),
            assets: HashMap::new(),
            created: now,
            modified: now,
        }
    }

    /// Get project name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get project version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get animations
    pub fn animations(&self) -> &HashMap<Uuid, ProjectAnimation> {
        &self.animations
    }

    /// Add animation to project
    pub fn add_animation(&mut self, name: &str) -> Uuid {
        let animation = ProjectAnimation::new(name);
        let id = animation.id;
        self.animations.insert(id, animation);
        self.modified = chrono::Utc::now();
        id
    }

    /// Remove animation from project
    pub fn remove_animation(&mut self, id: Uuid) -> Result<()> {
        if self.animations.remove(&id).is_some() {
            self.modified = chrono::Utc::now();
            Ok(())
        } else {
            Err(StudioError::ProjectError(format!(
                "Animation {} not found",
                id
            )))
        }
    }

    /// Get animation by ID
    pub fn get_animation(&self, id: Uuid) -> Option<&ProjectAnimation> {
        self.animations.get(&id)
    }

    /// Get mutable animation by ID
    pub fn get_animation_mut(&mut self, id: Uuid) -> Option<&mut ProjectAnimation> {
        if let Some(animation) = self.animations.get_mut(&id) {
            self.modified = chrono::Utc::now();
            Some(animation)
        } else {
            None
        }
    }

    /// Add asset to project
    pub fn add_asset(&mut self, asset: ProjectAsset) -> Uuid {
        let id = asset.id;
        self.assets.insert(id, asset);
        self.modified = chrono::Utc::now();
        id
    }

    /// Remove asset from project
    pub fn remove_asset(&mut self, id: Uuid) -> bool {
        if self.assets.remove(&id).is_some() {
            self.modified = chrono::Utc::now();
            true
        } else {
            false
        }
    }
}

/// Project settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Target framerate
    pub framerate: f32,
    /// Canvas dimensions
    pub canvas_width: u32,
    pub canvas_height: u32,
    /// Background color
    pub background_color: [u8; 4],
    /// Grid settings
    pub grid_enabled: bool,
    pub grid_size: f32,
    /// Snap settings
    pub snap_enabled: bool,
    pub snap_to_grid: bool,
    /// Performance settings
    pub webgl_enabled: bool,
    pub gpu_acceleration: bool,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            framerate: 60.0,
            canvas_width: 1920,
            canvas_height: 1080,
            background_color: [30, 30, 30, 255],
            grid_enabled: true,
            grid_size: 20.0,
            snap_enabled: true,
            snap_to_grid: true,
            webgl_enabled: true,
            gpu_acceleration: true,
        }
    }
}

/// Animation within a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnimation {
    /// Animation ID
    pub id: Uuid,
    /// Animation name
    pub name: String,
    /// Animation type
    pub animation_type: AnimationType,
    /// Timeline data
    pub timeline: Option<Timeline3D>,
    /// Transform data for 3D animations
    pub transforms: Vec<Transform3D>,
    /// Duration in seconds
    pub duration: f32,
    /// Is animation enabled
    pub enabled: bool,
    /// Animation tags
    pub tags: Vec<String>,
}

impl ProjectAnimation {
    /// Create new project animation
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            animation_type: AnimationType::Timeline,
            timeline: None,
            transforms: Vec::new(),
            duration: 5.0,
            enabled: true,
            tags: Vec::new(),
        }
    }
}

/// Animation type in project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    Timeline,
    Transform3D,
    PathMorphing,
    Procedural,
}

/// Project asset (images, SVGs, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAsset {
    /// Asset ID
    pub id: Uuid,
    /// Asset name
    pub name: String,
    /// Asset type
    pub asset_type: AssetType,
    /// File path or data URL
    pub source: String,
    /// Asset metadata
    pub metadata: HashMap<String, String>,
}

impl ProjectAsset {
    /// Create new asset
    pub fn new(name: String, asset_type: AssetType, source: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            asset_type,
            source,
            metadata: HashMap::new(),
        }
    }
}

/// Asset types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Video,
    Audio,
    SvgPath,
    Font,
    Script,
    Other(String),
}

/// Project manager for handling multiple projects
#[derive(Debug, Clone)]
pub struct ProjectManager {
    /// Currently loaded projects
    projects: HashMap<Uuid, StudioProject>,
    /// Currently active project
    active_project: Option<Uuid>,
}

impl ProjectManager {
    /// Create new project manager
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            active_project: None,
        }
    }

    /// Create new project
    pub fn create_project(&mut self, name: &str) -> Uuid {
        let project = StudioProject::new(name);
        let id = project.id;
        self.projects.insert(id, project);
        self.active_project = Some(id);
        id
    }

    /// Load project from data
    pub fn load_project(&mut self, project: StudioProject) -> Uuid {
        let id = project.id;
        self.projects.insert(id, project);
        id
    }

    /// Get project by ID
    pub fn get_project(&self, id: Uuid) -> Option<&StudioProject> {
        self.projects.get(&id)
    }

    /// Get mutable project by ID
    pub fn get_project_mut(&mut self, id: Uuid) -> Option<&mut StudioProject> {
        self.projects.get_mut(&id)
    }

    /// Set active project
    pub fn set_active_project(&mut self, id: Uuid) -> Result<()> {
        if self.projects.contains_key(&id) {
            self.active_project = Some(id);
            Ok(())
        } else {
            Err(StudioError::ProjectError(format!(
                "Project {} not found",
                id
            )))
        }
    }

    /// Get active project
    pub fn get_active_project(&self) -> Option<&StudioProject> {
        self.active_project.and_then(|id| self.projects.get(&id))
    }

    /// Get mutable active project
    pub fn get_active_project_mut(&mut self) -> Option<&mut StudioProject> {
        self.active_project
            .and_then(move |id| self.projects.get_mut(&id))
    }

    /// Remove project
    pub fn remove_project(&mut self, id: Uuid) -> bool {
        if self.active_project == Some(id) {
            self.active_project = None;
        }
        self.projects.remove(&id).is_some()
    }

    /// List all projects
    pub fn list_projects(&self) -> Vec<&StudioProject> {
        self.projects.values().collect()
    }

    /// Save project to JSON
    pub fn save_project_json(&self, id: Uuid) -> Result<String> {
        if let Some(project) = self.projects.get(&id) {
            serde_json::to_string_pretty(project)
                .map_err(|e| StudioError::ProjectError(format!("Serialization error: {}", e)))
        } else {
            Err(StudioError::ProjectError(format!(
                "Project {} not found",
                id
            )))
        }
    }

    /// Load project from JSON
    pub fn load_project_json(&mut self, json: &str) -> Result<Uuid> {
        let project: StudioProject = serde_json::from_str(json)
            .map_err(|e| StudioError::ProjectError(format!("Deserialization error: {}", e)))?;

        Ok(self.load_project(project))
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = StudioProject::new("Test Project");
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.version, "1.0.0");
        assert!(project.animations.is_empty());
        assert!(project.assets.is_empty());
    }

    #[test]
    fn test_project_animation_management() {
        let mut project = StudioProject::new("Test Project");

        let anim_id = project.add_animation("Test Animation");
        assert_eq!(project.animations.len(), 1);
        assert!(project.get_animation(anim_id).is_some());

        assert!(project.remove_animation(anim_id).is_ok());
        assert_eq!(project.animations.len(), 0);
    }

    #[test]
    fn test_project_manager() {
        let mut manager = ProjectManager::new();
        assert!(manager.get_active_project().is_none());

        let project_id = manager.create_project("Test Project");
        assert!(manager.get_active_project().is_some());
        assert_eq!(manager.get_active_project().unwrap().name, "Test Project");

        assert!(manager.remove_project(project_id));
        assert!(manager.get_active_project().is_none());
    }

    #[test]
    fn test_project_serialization() {
        let mut project = StudioProject::new("Test Project");
        project.add_animation("Test Animation");

        let mut manager = ProjectManager::new();
        let project_id = manager.load_project(project);

        let json = manager.save_project_json(project_id).unwrap();
        assert!(!json.is_empty());

        let loaded_id = manager.load_project_json(&json).unwrap();
        assert_ne!(project_id, loaded_id); // Different ID but same content

        let loaded_project = manager.get_project(loaded_id).unwrap();
        assert_eq!(loaded_project.name, "Test Project");
        assert_eq!(loaded_project.animations.len(), 1);
    }

    #[test]
    fn test_project_assets() {
        let mut project = StudioProject::new("Test Project");

        let asset = ProjectAsset::new(
            "logo.png".to_string(),
            AssetType::Image,
            "/assets/logo.png".to_string(),
        );

        let asset_id = project.add_asset(asset);
        assert_eq!(project.assets.len(), 1);

        assert!(project.remove_asset(asset_id));
        assert_eq!(project.assets.len(), 0);
    }
}
