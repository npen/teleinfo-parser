/*
 * teleinfo-parser
 * Copyright (c) 2018, 2019 Nicolas PENINGUY.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate matches;

/// Low level Teleinfo frame parsing.
pub mod frame;

/// Extracting information for the HC OptionTarifaire
pub mod hc;
