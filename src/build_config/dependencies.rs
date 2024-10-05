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
use std::{collections::HashSet, process::Command};
use toml::Spanned;

use super::error::{AdditionalInfo, Error, ErrorType};

// External dependencies (remote packages with versioning)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dependencies {
    pub remote: Vec<Spanned<RemoteDependency>>,
    pub pkg_config: Vec<Spanned<PkgConfigDependency>>,
    pub manual: Vec<Spanned<ManualDependency>>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Dependency {
    Remote(Spanned<RemoteDependency>),
    PkgConfig(Spanned<PkgConfigDependency>),
    Manual(Spanned<ManualDependency>),
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteBuildMethod {
    HeaderOnly,
    Cmake,
    Meson,
    Iceforge,
    Custom,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemoteDependency {
    pub name: Spanned<String>,
    pub version: Option<Spanned<String>>,
    pub source: Spanned<String>,
    pub include_name: Option<Spanned<String>>,
    pub include_dirs: Vec<String>,
    pub build_method: Option<RemoteBuildMethod>,
    pub build_command: Option<Spanned<String>>,
    pub build_output: Option<Spanned<String>>,
    pub imports: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PkgConfigDependency {
    pub name: Spanned<String>,
    pub pkg_config_query: Spanned<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ManualDependency {
    pub name: Spanned<String>,
    pub cflags: Option<String>,
    pub ldflags: Option<String>,
}

impl Iterator for Dependencies {
    type Item = Dependency;

    fn next(&mut self) -> Option<Self::Item> {
        // First check remote dependencies
        if let Some(remote) = self.remote.pop() {
            return Some(Dependency::Remote(remote));
        }
        // Then check pkg-config dependencies
        if let Some(pkg_config) = self.pkg_config.pop() {
            return Some(Dependency::PkgConfig(pkg_config));
        }
        // Finally check manual dependencies
        if let Some(manual) = self.manual.pop() {
            return Some(Dependency::Manual(manual));
        }
        None
    }
}

impl Dependencies {
    pub fn has_dependency(&self, name: &str) -> bool {
        for dep in self.clone() {
            match dep {
                Dependency::Remote(dep) => {
                    if dep.into_inner().name.into_inner() == name {
                        return true;
                    }
                }
                Dependency::PkgConfig(dep) => {
                    if dep.into_inner().name.into_inner() == name {
                        return true;
                    }
                }
                Dependency::Manual(dep) => {
                    if dep.into_inner().name.into_inner() == name {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn check_dependencies(&self) -> Result<(), Error> {
        // NOTE: Dependencies
        // Verify duplicate dependencies are not present
        // Verify no two dependencies share the same name or include_name
        // Verify that build_command, build_output are present only in custom build_method
        // Verify that pkg-config dependency exists

        #[derive(Eq, PartialEq, Hash, Clone)]
        struct RemoteInfo {
            url: Spanned<String>,
            version: Option<Spanned<String>>,
        }

        let mut url_set: HashSet<RemoteInfo> = HashSet::new();
        let mut name_set: HashSet<Spanned<String>> = HashSet::new();
        let mut include_name_set: HashSet<Spanned<String>> = HashSet::new();
        for dep in self.clone() {
            match dep {
                Dependency::Remote(remote) => {
                    let remote_info = RemoteInfo {
                        url: remote.clone().into_inner().source.clone(),
                        version: remote.clone().into_inner().version,
                    };
                    if !url_set.insert(remote_info.clone()) {
                        return Err(Error {
                            error_type: ErrorType::DuplicateDependencySource,
                            message: "Duplicate dependency url with same versions".to_string(),
                            span: Some(remote.into_inner().source.clone().span()),
                            additional_info: Some(AdditionalInfo {
                                message: "Previously defined here".to_string(),
                                span: url_set.get(&remote_info).unwrap().url.span(),
                            }),
                        });
                    }
                    if !name_set.insert(remote.clone().into_inner().name.clone()) {
                        return Err(Error {
                            error_type: ErrorType::DuplicateDependencyName,
                            message: "Duplicate dependency name".to_string(),
                            span: Some(remote.clone().into_inner().name.clone().span()),
                            additional_info: Some(AdditionalInfo {
                                message: "Previously defined here".to_string(),
                                span: name_set.get(&remote.into_inner().name).unwrap().span(),
                            }),
                        });
                    }
                    if let Some(include_name) = remote.clone().into_inner().include_name {
                        if !include_name_set.insert(include_name.clone()) {
                            return Err(Error {
                                error_type: ErrorType::DuplicateDependencyIncludeName,
                                message: "Duplicate dependency include name".to_string(),
                                span: Some(include_name.clone().span()),
                                additional_info: Some(AdditionalInfo {
                                    message: "Previously defined here".to_string(),
                                    span: include_name_set.get(&include_name).unwrap().span(),
                                }),
                            });
                        }
                    }

                    if let Some(build_method) = remote.clone().into_inner().build_method {
                        if build_method == RemoteBuildMethod::Custom {
                            if remote.clone().into_inner().build_command.is_none() {
                                return Err(Error {
                                    error_type: ErrorType::CustomBuildMissing,
                                    message: "Custom build method missing build_command"
                                        .to_string(),
                                    span: Some(remote.span()),
                                    additional_info: None,
                                });
                            }
                        } else {
                            if let Some(build_output) = remote.clone().into_inner().build_output {
                                return Err(Error {
                                    error_type: ErrorType::ExtraFieldNonCustomBuild,
                                    message: "Non-Custom build method has build_output".to_string(),
                                    span: Some(build_output.span()),
                                    additional_info: None,
                                });
                            }
                            if let Some(build_command) = remote.clone().into_inner().build_command {
                                return Err(Error {
                                    error_type: ErrorType::ExtraFieldNonCustomBuild,
                                    message: "non-Custom build method has build_command"
                                        .to_string(),
                                    span: Some(build_command.span()),
                                    additional_info: None,
                                });
                            }
                        }
                    }
                }
                Dependency::PkgConfig(pkg_config) => {
                    if !name_set.insert(pkg_config.clone().into_inner().name.clone()) {
                        return Err(Error {
                            error_type: ErrorType::DuplicateDependencyName,
                            message: "Duplicate dependency name".to_string(),
                            span: Some(pkg_config.clone().into_inner().name.clone().span()),
                            additional_info: Some(AdditionalInfo {
                                message: "Previously defined here".to_string(),
                                span: name_set.get(&pkg_config.into_inner().name).unwrap().span(),
                            }),
                        });
                    }

                    // Check if pkg-config dependency exists
                    let status = Command::new("pkg-config")
                        .arg("--exists")
                        .arg(
                            pkg_config
                                .clone()
                                .into_inner()
                                .pkg_config_query
                                .into_inner(),
                        )
                        .status();
                    if status.is_err() || status.unwrap().code() != Some(0) {
                        return Err(Error {
                            error_type: ErrorType::InvalidPkgConfigQuery,
                            message: "Pkg-config dependency not found".to_string(),
                            span: Some(pkg_config.into_inner().pkg_config_query.clone().span()),
                            additional_info: None,
                        });
                    }
                }
                Dependency::Manual(manual) => {
                    if !name_set.insert(manual.clone().into_inner().name.clone()) {
                        return Err(Error {
                            error_type: ErrorType::DuplicateDependencyName,
                            message: "Duplicate dependency name".to_string(),
                            span: Some(manual.clone().into_inner().name.span()),
                            additional_info: Some(AdditionalInfo {
                                message: "Previously defined here".to_string(),
                                span: name_set.get(&manual.into_inner().name).unwrap().span(),
                            }),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}
