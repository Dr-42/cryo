/*
* Copyright (c) 2024, Dr. Spandan Roy
*
* This file is part of cryo.
*
* cryo is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* cryo is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with cryo.  If not, see <https://www.gnu.org/licenses/>.
*/

use serde::{Deserialize, Serialize};
use std::fs;
use toml::de::Error; // For handling deserialization errors

// Main struct representing the entire configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildConfig {
    pub build: BuildSettings,
    pub dependencies: Dependencies,
    pub subprojects: Vec<SubProject>,
    pub custom_build_rules: Option<Vec<CustomBuildRule>>,
    pub build_overrides: Option<BuildOverrides>,
}

// General build configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildSettings {
    pub c_standard: String,
    pub compiler: String,
    pub optimization_flags: Option<String>,
    pub parallel_jobs: Option<u32>,
    pub target: Option<String>,
    pub output_dir: String,
}

// External dependencies (remote packages with versioning)
#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    pub remote: Vec<RemoteDependency>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteDependency {
    pub name: String,
    pub version: String,
    pub source: String,
    pub include_dir: Option<String>,
    pub lib_dir: Option<String>,
}

// Subprojects (binaries, libraries, or header-only)
#[derive(Debug, Deserialize, Serialize)]
pub struct SubProject {
    pub name: String,
    pub r#type: SubProjectType, // Enum to specify the type (binary, library, header-only)
    pub src_dir: Option<String>,
    pub include_dirs: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub output_name: Option<String>,
}

// Enum for subproject type
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")] // Matches the TOML string "binary", "library", "header-only"
pub enum SubProjectType {
    Binary,
    Library,
    HeaderOnly,
}

// Custom build rules for assets like Vulkan shaders
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomBuildRule {
    pub name: String,
    pub description: Option<String>,
    pub src_dir: String,
    pub output_dir: String,
    pub trigger_extensions: Vec<String>,
    pub command: String,
    pub rebuild_if_changed: Option<bool>,
}

// Build overrides for specific subprojects (optional)
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildOverrides {
    pub core: Option<SubProjectBuildOverride>,
    pub game: Option<SubProjectBuildOverride>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubProjectBuildOverride {
    pub c_standard: Option<String>,
    pub compiler_flags: Option<String>,
}

fn load_config(file_path: &str) -> Result<BuildConfig, Error> {
    // Read the TOML file
    let content = fs::read_to_string(file_path).expect("Failed to read the config file");

    // Parse the TOML content into the BuildConfig struct
    let config: BuildConfig = toml::from_str(&content)?;

    Ok(config)
}

fn main() {
    match load_config("sample.toml") {
        Ok(config) => println!("{:#?}", config),
        Err(e) => eprintln!("Error parsing config: {}", e),
    }
}
