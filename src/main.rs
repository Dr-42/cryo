/*
* Copyright (c) 2024, Dr. Spandan Roy
*
* This file is part of cryo.
*
* cryo is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* cryo is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with cryo.  If not, see <https://www.gnu.org/licenses/>.
*/

pub mod build_config;
pub mod cli;

fn main() {
    let _config = match build_config::BuildConfig::load_config("sample.toml") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing config: {}", e);
            std::process::exit(1);
        }
    };
    cli::parse();
}
