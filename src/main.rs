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

pub mod build_config;
pub mod cli;
pub mod error;
pub mod logger;
pub mod package;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "sample.toml";
    let mut config = match build_config::BuildConfig::load_config(config_path) {
        Ok(config) => config,
        Err(e) => {
            e.emit_config_error(config_path);
            std::process::exit(1);
        }
    };
    if let Err(e) = config.verify_config() {
        e.emit_config_error(config_path);
        std::process::exit(1);
    }
    cli::parse();
    Ok(())
}
