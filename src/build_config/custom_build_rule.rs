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
