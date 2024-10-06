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
use super::{dependencies::Dependencies, Error};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use toml::Spanned;

use crate::error::{AdditionalInfo, ErrorType};

// Enum for subproject type
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
    pub name: Spanned<String>,
    pub r#type: SubProjectType,
    pub src_dir: Option<String>,
    pub include_dirs: Option<Vec<String>>,
    pub dependencies: Option<Vec<Spanned<SubProjectDependency>>>,
}

impl SubProject {
    fn check_duplicate_names(selfs: Vec<Self>) -> Result<HashSet<String>, Error> {
        let mut name_set = HashSet::new();
        let mut lib_set = HashSet::new();
        for subproject in selfs.clone() {
            if !name_set.insert(subproject.name.clone()) {
                return Err(Error {
                    error_type: ErrorType::DuplicateSubprojectName,
                    message: format!(
                        "Duplicate subproject name: {}",
                        subproject.name.clone().into_inner()
                    ),
                    span: Some(subproject.name.span()),
                    additional_info: Some(AdditionalInfo {
                        span: name_set
                            .get(&subproject.name.clone())
                            .unwrap()
                            .span()
                            .clone(),
                        message: format!(
                            "Previous subproject with same name: {}",
                            subproject.name.clone().into_inner()
                        ),
                    }),
                });
            } else if subproject.r#type == SubProjectType::Library
                || subproject.r#type == SubProjectType::HeaderOnly
            {
                lib_set.insert(subproject.name.clone().into_inner());
            }
        }
        Ok(lib_set)
    }

    fn check_subproject_dependencies(
        selfs: &[Self],
        dependencies: &Dependencies,
        lib_set: &HashSet<String>,
    ) -> Result<(), Error> {
        for subproject in selfs {
            if let Some(deps) = &subproject.dependencies {
                for dep in deps {
                    let dep_span = dep.span();
                    let dep = dep.clone().into_inner();
                    match dep {
                        SubProjectDependency::Named(name) => {
                            if !dependencies.has_dependency(&name.clone())
                                && !lib_set.contains(&name.clone())
                            {
                                return Err(Error {
                                    error_type: ErrorType::InvalidSubprojectDependency,
                                    message: format!("Invalid dependency: {}", name.clone()),
                                    span: Some(dep_span),
                                    additional_info: None,
                                });
                            }
                        }
                        SubProjectDependency::Detailed { name, .. } => {
                            if dependencies.has_dependency(&name.clone()) {
                                // TODO: Grab individual imports from remote dependencies
                                return Ok(());
                            } else if !lib_set.contains(&name.clone()) {
                                return Err(Error {
                                    error_type: ErrorType::InvalidSubprojectDependency,
                                    message: format!("Invalid dependency: {}", name.clone()),
                                    span: Some(dep_span),
                                    additional_info: None,
                                });
                            } else {
                                unreachable!("How did we get here?");
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn dfs_cycle_detection(
        project: &String,
        dependency_map: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
        path: &mut Vec<String>, // Add this to track the path
    ) -> Result<(), String> {
        if stack.contains(project) {
            // Circular dependency detected
            path.push(project.clone()); // Push the project to the path
            return Err(path.join(" -> ")); // Return the circular path as error message
        }

        if !visited.contains(project) {
            // Mark the current project as visited and add to the recursion stack
            visited.insert(project.clone());
            stack.insert(project.clone());
            path.push(project.clone()); // Track the path

            // Recur for all dependencies (adjacent nodes)
            if let Some(dependencies) = dependency_map.get(project) {
                for dep in dependencies {
                    Self::dfs_cycle_detection(dep, dependency_map, visited, stack, path)?
                }
            }

            // Remove from recursion stack and path once processed
            stack.remove(project);
            path.pop(); // Remove the project from the path
        }

        Ok(())
    }

    // Function to perform topological sort using DFS
    fn dfs_topological_sort(
        project: &String,
        dependency_map: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) {
        if !visited.contains(project) {
            visited.insert(project.clone());

            // Recur for all dependencies (adjacent nodes)
            if let Some(dependencies) = dependency_map.get(project) {
                for dep in dependencies {
                    Self::dfs_topological_sort(dep, dependency_map, visited, order);
                }
            }

            // Push the current project to the build order after all dependencies are processed
            order.push(project.clone());
        }
    }

    // Function to check for circular dependencies and return a valid build order
    fn check_circular_dependencies_and_get_build_order(
        selfs: &[SubProject],
    ) -> Result<Vec<SubProject>, Error> {
        // Step 1: Construct the dependency graph
        let dependency_map: HashMap<String, Vec<String>> = selfs
            .iter()
            .map(|subproject| {
                let deps = if let Some(dep_list) = &subproject.dependencies {
                    dep_list
                        .iter()
                        .map(|dep| match dep.clone().into_inner() {
                            SubProjectDependency::Named(name) => name,
                            SubProjectDependency::Detailed { name, .. } => name,
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                (subproject.name.clone().into_inner(), deps)
            })
            .collect();

        // Step 2: Prepare sets to track visited nodes and the recursion stack
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();

        // Step 3: Run DFS for each subproject to detect cycles
        for subproject in selfs {
            let project_name = subproject.name.clone().into_inner();
            let mut path = Vec::new(); // Track the cycle path here

            if !visited.contains(&project_name) {
                if let Err(cycle_path) = Self::dfs_cycle_detection(
                    &project_name,
                    &dependency_map,
                    &mut visited,
                    &mut stack,
                    &mut path,
                ) {
                    return Err(Error {
                        error_type: ErrorType::CircularDependency,
                        message: format!(
                            "Circular dependency detected in subproject: {}",
                            project_name
                        ),
                        span: Some(subproject.name.span()),
                        additional_info: Some(AdditionalInfo {
                            span: subproject.name.span(),
                            message: format!("Dependency cycle: {}", cycle_path), // Add the cycle path here
                        }),
                    });
                }
            }
        }

        // Step 4: Now that we know there's no circular dependency, generate the build order
        let mut topological_order = Vec::new();
        let mut visited = HashSet::new();

        // Run DFS again for topological sorting
        for subproject in selfs {
            let project_name = subproject.name.clone().into_inner();
            if !visited.contains(&project_name) {
                Self::dfs_topological_sort(
                    &project_name,
                    &dependency_map,
                    &mut visited,
                    &mut topological_order,
                );
            }
        }

        // Step 5: Reverse the topological order (DFS will give us the reverse order)
        topological_order.reverse();

        // Step 6: Map the topological order back to the corresponding subprojects
        let build_order = topological_order
            .into_iter()
            .filter_map(|name| {
                selfs
                    .iter()
                    .find(|subproject| subproject.name.clone().into_inner() == name)
                    .cloned()
            })
            .collect::<Vec<_>>();

        Ok(build_order)
    }

    pub fn verify_subprojects(
        selfs: Vec<Self>,
        dependencies: &Dependencies,
    ) -> Result<Vec<Self>, Error> {
        // NOTE: Subprojects
        // Verify duplicate subproject names are not present
        // Verify that subproject dependencies exist
        // Verify that there are no circular dependencies
        let name_set = Self::check_duplicate_names(selfs.clone())?;
        // TODO: Verify that src_dir and include_dirs exist (except in header_only)
        // TODO: Grab all remote dependencies as they are needed to verify subproject dependencies
        Self::check_subproject_dependencies(&selfs, dependencies, &name_set)?;
        Self::check_circular_dependencies_and_get_build_order(&selfs)
    }
}
