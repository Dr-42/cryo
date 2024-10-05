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
use std::{collections::HashSet, fs, ops::Range, process::Command};
use toml::{de::Error as TomlError, Spanned}; // For handling deserialization errors

// Main struct representing the entire configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildConfig {
    pub build: BuildSettings,
    pub dependencies: Dependencies,
    pub subprojects: Vec<SubProject>,
    pub custom_build_rules: Option<Vec<CustomBuildRule>>,
    pub overrides: Option<Vec<Override>>,
}

// General build configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildSettings {
    pub version: String,
    pub c_standard: Spanned<String>,
    pub compiler: Spanned<String>,
    pub global_cflags: Option<String>,
    pub debug_flags: Option<String>,
    pub release_flags: Option<String>,
    pub parallel_jobs: Option<u32>,
}

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

// Overrides
#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub name: String,
    pub description: Option<String>,
    pub src_dir: String,
    pub output_dir: String,
    pub trigger_extensions: Vec<String>,
    pub output_extension: String,
    pub command: String,
    pub rebuild_rule: CustomBuildRuleType,
}

// Additional information
#[derive(Debug)]
pub struct AdditionalInfo {
    pub span: Range<usize>,
    pub message: String,
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
    pub span: Option<Range<usize>>,
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug)]
pub enum ErrorType {
    TomlParseError,
    IncorrectCompiler,
    UnsupportedCStandard,
    DuplicateDependencySource,
    DuplicateDependencyName,
    DuplicateDependencyIncludeName,
    CustomBuildMissing,
    ExtraFieldNonCustomBuild,
    InvalidPkgConfigQuery,
}

impl BuildSettings {
    fn check_compiler_details(&self) -> Result<(), Error> {
        // NOTE: Compiler details
        // Check if the compiler is in the path
        // Check if the standard is supported
        let compiler = self.compiler.clone();
        let compiler_span = compiler.span();
        let compiler_name = compiler.into_inner();

        // Check if the compiler is in the path
        let compiler_path = Command::new("sh")
            .arg("-c")
            .arg(format!("which {}", compiler_name))
            .output();
        let compiler_path = if let Ok(compiler_path) = compiler_path {
            let output = String::from_utf8(compiler_path.stdout).unwrap();
            let output = output.split_whitespace().next();
            if let Some(output) = output {
                output.to_string()
            } else {
                return Err(Error {
                    error_type: ErrorType::IncorrectCompiler,
                    message: "Compiler not in path".to_string(),
                    span: Some(compiler_span),
                    additional_info: None,
                });
            }
        } else {
            return Err(Error {
                error_type: ErrorType::IncorrectCompiler,
                message: "Compiler not in path".to_string(),
                span: Some(compiler_span),
                additional_info: None,
            });
        };
        let c_standard = self.c_standard.clone();
        let c_standard_span = c_standard.span();
        let c_standard = c_standard.into_inner();
        let output = Command::new(compiler_path)
            .arg(format!("-std={}", c_standard))
            .arg("-o") // Dummy output
            .arg("/dev/null") // Just discard any output file
            .arg("-x") // Specify language C
            .arg("c") // Use C language
            .arg("-c") // Compile only, don't link
            .arg("-") // Read from stdin
            .output();

        if output.is_err() || output.unwrap().status.code() != Some(0) {
            return Err(Error {
                error_type: ErrorType::UnsupportedCStandard,
                message: "Unsupported C standard".to_string(),
                span: Some(c_standard_span),
                additional_info: None,
            });
        }
        Ok(())
    }
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
    fn check_dependencies(&self) -> Result<(), Error> {
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
