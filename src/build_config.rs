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
use toml::de::Error as TomlError; // For handling deserialization errors

mod build_settings;
mod custom_build_rule;
mod dependencies;
mod error;
mod r#override;
mod subproject;

use build_settings::BuildSettings;
use custom_build_rule::CustomBuildRule;
use dependencies::Dependencies;
use error::{Error, ErrorType};
use r#override::Override;
use subproject::SubProject;

// Main struct representing the entire configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildConfig {
    pub build: BuildSettings,
    pub dependencies: Dependencies,
    pub subprojects: Vec<SubProject>,
    pub custom_build_rules: Option<Vec<CustomBuildRule>>,
    pub overrides: Option<Vec<Override>>,
}

impl BuildConfig {
    pub fn load_config(file_path: &str) -> Result<Self, Error> {
        // Read the TOML file
        let content = fs::read_to_string(file_path).expect("Failed to read the config file");
        // Parse the TOML content into the BuildConfig struct
        let config: Result<Self, TomlError> = toml::from_str(&content);
        match config {
            Err(e) => Err(Error {
                error_type: ErrorType::TomlParseError,
                message: e.to_string(),
                span: e.span(),
                additional_info: None,
            }),
            Ok(config) => Ok(config),
        }
    }

    pub fn verify_config(&self) -> Result<(), Error> {
        self.build.check_compiler_details()?;
        self.dependencies.check_dependencies()?;
        SubProject::verify_subprojects(self.subprojects.clone())?;

        // NOTE: Overrrides
        // TODO: Verify duplicate override names are not present
        // Verify that override names match subproject names
        //
        // NOTE: Custom build rules
        // TODO: Verify duplicate custom build rule names are not present
        // Verify that src_dir and output_dir exist
        todo!()
    }
}
