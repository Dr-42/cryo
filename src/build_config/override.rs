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
