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
use toml::Spanned;

use crate::error::{AdditionalInfo, Error, ErrorType};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum CustomBuildRuleType {
    IfChanged,
    Always,
    OnTrigger,
}

// Custom build rules for assets like Vulkan shaders
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomBuildRule {
    pub name: Spanned<String>,
    pub description: Option<String>,
    pub src_dir: String,
    pub output_dir: String,
    pub trigger_extensions: Vec<String>,
    pub output_extension: String,
    pub command: String,
    pub rebuild_rule: CustomBuildRuleType,
}

impl CustomBuildRule {
    pub fn verify_custom_build_rules(selfs: &[Self]) -> Result<(), Error> {
        // NOTE: Custom build rules
        // Verify duplicate custom build rule names are not present
        let mut name_set = std::collections::HashSet::new();

        for cbr in selfs {
            if !name_set.insert(cbr.name.clone()) {
                return Err(Error {
                    error_type: ErrorType::DuplicateCustomBuildRuleName,
                    message: format!(
                        "Duplicate custom build rule name {}",
                        cbr.name.clone().into_inner()
                    ),
                    span: Some(cbr.name.span()),
                    additional_info: Some(AdditionalInfo {
                        span: name_set.get(&cbr.name).unwrap().span(),
                        message: "Previous definition".to_string(),
                    }),
                });
            }
        }
        //  TODO: Verify that src_dir and output_dir exist

        Ok(())
    }
}
