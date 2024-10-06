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
use std::ops::Range;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

// Additional information
#[derive(Debug, Clone)]
pub struct AdditionalInfo {
    pub span: Range<usize>,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
    pub span: Option<Range<usize>>,
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug, Clone)]
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
    DuplicateSubprojectName,
    InvalidSubprojectDependency,
    CircularDependency,
    OverrideNameConflict,
    DuplicateCustomBuildRuleName,
}

impl Error {
    pub fn emit_config_error(&self, config_path: &str) {
        let config_contents = std::fs::read_to_string(config_path).unwrap();
        let mut files = SimpleFiles::new();
        let file_id = files.add(config_path, config_contents);
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        let mut labels_vec = Vec::new();

        labels_vec.push(
            Label::primary(file_id, self.span.clone().unwrap()).with_message(self.clone().message),
        );
        if let Some(additional_info) = self.additional_info.clone() {
            labels_vec.push(
                Label::secondary(file_id, additional_info.span)
                    .with_message(additional_info.message),
            );
        }

        let diag = Diagnostic::error()
            .with_message("Error parsing config")
            .with_labels(labels_vec);

        term::emit(&mut writer.lock(), &config, &files, &diag).unwrap();
    }
}
