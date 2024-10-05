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
use std::process::Command;
use toml::Spanned;

use super::{Error, ErrorType};

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

impl BuildSettings {
    pub fn check_compiler_details(&self) -> Result<(), Error> {
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
