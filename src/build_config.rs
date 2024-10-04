/*
* Copyright (c) 2024, Dr. Spandan Roy
*
* This file is part of iceforge.
*
* iceforge is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* iceforge is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with iceforge.  If not, see <https://www.gnu.org/licenses/>.
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
    pub overrides: Option<Vec<Override>>,
}

// General build configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildSettings {
    pub version: String,
    pub c_standard: String,
    pub compiler: String,
    pub global_cflags: Option<String>,
    pub debug_flags: Option<String>,
    pub release_flags: Option<String>,
    pub parallel_jobs: Option<u32>,
}

// External dependencies (remote packages with versioning)
#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    pub remote: Vec<RemoteDependency>,
    pub pkg_config: Vec<PkgConfigDependency>,
    pub manual: Vec<ManualDependency>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteBuildMethod {
    HeaderOnly,
    Cmake,
    Meson,
    Iceforge,
    Custom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteDependency {
    pub name: String,
    pub version: Option<String>,
    pub source: String,
    pub include_name: Option<String>,
    pub include_dirs: Vec<String>,
    pub build_method: Option<RemoteBuildMethod>,
    pub build_command: Option<String>,
    pub build_output: Option<String>,
    pub imports: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PkgConfigDependency {
    pub name: String,
    pub pkg_config_query: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManualDependency {
    pub name: String,
    pub cflags: Option<String>,
    pub ldflags: Option<String>,
}

// Enum for subproject type
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")] // Matches the TOML string "binary", "library", "header-only"
pub enum SubProjectType {
    Binary,
    Library,
    HeaderOnly,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SubProjectDependency {
    Named(String), // For simple dependencies like "freetype"
    Detailed {
        name: String,
        imports: Option<Vec<String>>,
    },
}

// Subprojects (binaries, libraries, or header-only)
#[derive(Debug, Deserialize, Serialize)]
pub struct SubProject {
    pub name: String,
    pub r#type: SubProjectType, // Enum to specify the type (binary, library, header-only)
    pub src_dir: Option<String>,
    pub include_dirs: Option<Vec<String>>,
    pub dependencies: Option<Vec<SubProjectDependency>>,
}

// Overrides
#[derive(Debug, Deserialize, Serialize)]
pub struct Override {
    pub name: String,
    pub c_standard: Option<String>,
    pub compiler: Option<String>,
    pub cflags: Option<String>,
    pub debug_flags: Option<String>,
    pub release_flags: Option<String>,
    pub parallel_jobs: Option<u32>,
}

// Enum for subproject type
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")] // Matches the TOML string "binary", "library", "header-only"
pub enum CustomBuildRuleType {
    IfChanged,
    Always,
    OnTrigger,
}

// Custom build rules for assets like Vulkan shaders
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomBuildRule {
    pub name: String,
    pub description: Option<String>,
    pub src_dir: String,
    pub output_dir: String,
    pub trigger_extensions: Vec<String>,
    pub output_extension: String,
    pub command: String,
    pub rebuild_rule: CustomBuildRuleType,
}

#[derive(Debug)]
pub enum ConfigError {}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigError")
    }
}

impl std::error::Error for ConfigError {}

impl BuildConfig {
    pub fn load_config(file_path: &str) -> Result<Self, Error> {
        // Read the TOML file
        let content = fs::read_to_string(file_path).expect("Failed to read the config file");
        // Parse the TOML content into the BuildConfig struct
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    fn check_compiler_details(&self) {
        let compiler = self.build.compiler.as_str();
        let c_standard = self.build.c_standard.as_str();
    }

    pub fn verify_config(&self) -> Result<(), ConfigError> {
        // NOTE: Build settings
        // TODO: Verify compiler is in path
        // Verify that c_standard is in the list
        self.check_compiler_details();
        // NOTE: Dependencies
        // TODO: Verify duplicate dependencies are not present
        // Verify no two dependencies share the same name or include_name
        // Verify that build_command, build_output are present only in custom
        // build_method
        // Verify that pkg-config dependency exists
        //
        // NOTE: Subprojects
        // TODO: Verify duplicate subproject names are not present
        // Verify that subproject dependencies exist
        // Verify that there are no circular dependencies
        // Verify that src_dir and include_dirs exist (except in header_only)
        //
        // NOTE: Overrriders
        // TODO: Verify duplicate override names are not present
        // Verify that override names match subproject names
        //
        // NOTE: Custom build rules
        // TODO: Verify duplicate custom build rule names are not present
        // Verify that src_dir and output_dir exist
        todo!()
    }
}
