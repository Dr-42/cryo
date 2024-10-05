use std::collections::HashSet;

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

use super::{
    error::{AdditionalInfo, Error, ErrorType},
    subproject::SubProject,
};
// Overrides
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Override {
    pub name: Spanned<String>,
    pub c_standard: Option<String>,
    pub compiler: Option<String>,
    pub cflags: Option<String>,
    pub debug_flags: Option<String>,
    pub release_flags: Option<String>,
    pub parallel_jobs: Option<u32>,
}

impl Override {
    pub fn verify_overrides(selfs: &[Self], sub_projects: &[SubProject]) -> Result<(), Error> {
        // NOTE: Overrrides
        // Verify duplicate override names are not present
        // TODO: Verify that override names match subproject names
        let mut name_set = HashSet::new();

        for over in selfs {
            if !name_set.insert(over.name.clone()) {
                return Err(Error {
                    error_type: ErrorType::OverrideNameConflict,
                    message: format!(
                        "Override name {} is already in use",
                        over.name.clone().into_inner()
                    ),
                    span: Some(over.name.span()),
                    additional_info: Some(AdditionalInfo {
                        span: name_set.get(&over.name).unwrap().span(),
                        message: "Previous definition".to_string(),
                    }),
                });
            }
        }

        for name in name_set {
            if !sub_projects
                .iter()
                .any(|sub| sub.name.clone().into_inner() == name.clone().into_inner())
            {
                return Err(Error {
                    error_type: ErrorType::OverrideNameConflict,
                    message: format!(
                        "Override name {} is not defined in any subproject",
                        name.clone().into_inner()
                    ),
                    span: Some(name.span()),
                    additional_info: None,
                });
            }
        }
        Ok(())
    }
}
