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
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

pub mod build_config;
pub mod cli;
pub mod logger;

fn emit_config_error(e: build_config::Error) {
    let config_path = "sample.toml";
    let config_contents = std::fs::read_to_string(config_path).unwrap();
    let mut files = SimpleFiles::new();
    let file_id = files.add(config_path, config_contents);
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let mut labels_vec = Vec::new();

    labels_vec.push(Label::primary(file_id, e.span.unwrap()).with_message(e.message));
    if let Some(additional_info) = e.additional_info {
        labels_vec.push(
            Label::secondary(file_id, additional_info.span).with_message(additional_info.message),
        );
    }

    let diag = Diagnostic::error()
        .with_message("Error parsing config")
        .with_labels(labels_vec);

    term::emit(&mut writer.lock(), &config, &files, &diag).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "sample.toml";
    let config = match build_config::BuildConfig::load_config(config_path) {
        Ok(config) => config,
        Err(e) => {
            emit_config_error(e);
            std::process::exit(1);
        }
    };
    if let Err(e) = config.verify_config() {
        emit_config_error(e);
        std::process::exit(1);
    }
    cli::parse();
    Ok(())
}
