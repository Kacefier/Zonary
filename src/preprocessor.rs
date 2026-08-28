//! Zonary Interpreter - Made by Kacefier - Version 2.1.1 - Preprocessor directive handling
//!
//! Copyright (C) 2026 Kacefier
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum PreprocessorDirective {
    ReplaceChars(char, char),  // /00: replace binary characters
    SetBitWidth(u8),           // /01: set bit width
    Reserved,                  // /10 and /11: reserved for future use
}

impl PreprocessorDirective {
    // Parse a preprocessor directive string
    // Format: /XX params
    pub fn parse(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.is_empty() {
            return Err(anyhow!("Empty preprocessor directive"));
        }

        let directive = parts[0];

        match directive {
            "/00" => {
                // Replace characters: /00 zero one
                if parts.len() != 3 {
                    return Err(anyhow!("/00 requires exactly 2 parameters"));
                }
                let zero = Self::parse_ascii_char(parts[1])?;
                let one = Self::parse_ascii_char(parts[2])?;

                // Custom ASCII characters cannot be <, >, /, 0, or 1
                // nor can they be identical to each other
                if zero == one {
                    return Err(anyhow!("Custom characters cannot be identical"));
                }
                let reserved = ['<', '>', '/', '0', '1'];
                if reserved.contains(&zero) || reserved.contains(&one) {
                    return Err(anyhow!("Custom characters cannot be <, >, /, 0, or 1"));
                }

                Ok(PreprocessorDirective::ReplaceChars(zero, one))
            }
            "/01" => {
                // Set bit width: /01 width
                if parts.len() != 2 {
                    return Err(anyhow!("/01 requires exactly 1 parameter"));
                }
                let width = Self::parse_binary_u8(parts[1])?;
                if width < 2 {
                    return Err(anyhow!("Bit width must be at least 2"));
                }
                Ok(PreprocessorDirective::SetBitWidth(width))
            }
            "/10" | "/11" => {
                // Reserved for future use, no effect in this version
                Ok(PreprocessorDirective::Reserved)
            }
            _ => Err(anyhow!("Unknown preprocessor directive: {}", directive)),
        }
    }

    // Parse an 8-bit binary string to an ASCII character
    fn parse_ascii_char(s: &str) -> Result<char> {
        let bits = Self::parse_binary_u8(s)?;
        if bits < 32 || bits > 126 {
            return Err(anyhow!("ASCII character code must be between 32 and 126"));
        }
        Ok(bits as u8 as char)
    }

    // Parse an 8-bit binary string to a u8 value
    fn parse_binary_u8(s: &str) -> Result<u8> {
        if s.len() != 8 {
            return Err(anyhow!("Binary parameter must be exactly 8 bits"));
        }
        if !s.chars().all(|c| c == '0' || c == '1') {
            return Err(anyhow!("Parameter must be binary (0 and 1 only)"));
        }
        u8::from_str_radix(s, 2).map_err(|e| anyhow!("Failed to parse binary: {}", e))
    }
}

impl fmt::Display for PreprocessorDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreprocessorDirective::ReplaceChars(zero, one) => {
                write!(f, "/00 {:08b} {:08b}", *zero as u8, *one as u8)
            }
            PreprocessorDirective::SetBitWidth(width) => {
                write!(f, "/01 {:08b}", width)
            }
            PreprocessorDirective::Reserved => write!(f, "/10"),
        }
    }
}
