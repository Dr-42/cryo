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
use super::Error;
use serde::{Deserialize, Serialize};

// Enum for subproject type
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")] // Matches the TOML string "binary", "library", "header-only"
pub enum SubProjectType {
    Binary,
    Library,
    HeaderOnly,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum SubProjectDependency {
    Named(String),
    Detailed {
        name: String,
        imports: Option<Vec<String>>,
    },
}

// Subprojects (binaries, libraries, or header-only)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubProject {
    pub name: String,
    pub r#type: SubProjectType,
    pub src_dir: Option<String>,
    pub include_dirs: Option<Vec<String>>,
    pub dependencies: Option<Vec<SubProjectDependency>>,
}

impl SubProject {
    pub fn verify_subprojects(_vec: Vec<Self>) -> Result<(), Error> {
        // NOTE: Subprojects
        // TODO: Verify duplicate subproject names are not present
        // Verify that subproject dependencies exist
        // Verify that there are no circular dependencies
        // Verify that src_dir and include_dirs exist (except in header_only)
        todo!()
    }
}
