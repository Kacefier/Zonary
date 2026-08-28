//! Zonary Interpreter - Made by Kacefier - Version 2.1.1 - Core interpreter logic
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

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::preprocessor::PreprocessorDirective;
use crate::vm::{Instruction, VM, DEFAULT_BIT_WIDTH};

pub struct Interpreter {
    bit_width: u8,
    char_zero: char,
    char_one: char,
    labels: HashMap<u64, usize>,
    instructions: Vec<Instruction>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            bit_width: DEFAULT_BIT_WIDTH,
            char_zero: '0',
            char_one: '1',
            labels: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    pub fn run(&mut self, source: &str) -> Result<i32> {
        // Reset interpreter state
        self.bit_width = DEFAULT_BIT_WIDTH;
        self.char_zero = '0';
        self.char_one = '1';
        self.labels.clear();
        self.instructions.clear();

        // Phase 1: Extract and execute preprocessor directives
        let (source_without_directives, directives) = self.extract_directives(source)?;

        for directive in &directives {
            self.execute_directive(directive)?;
        }

        // Phase 2: Clean the source (remove comments, whitespace, convert custom chars)
        let cleaned_source = self.clean_source(&source_without_directives);

        // Phase 3: Parse the cleaned binary string into instructions
        let (instructions, label_defs) = self.parse_binary(&cleaned_source)?;
        self.instructions = instructions;
        self.labels = label_defs;

        // Phase 4: Execute
        let mut vm = VM::new(self.bit_width);
        vm.execute(&self.instructions, &self.labels)?;

        Ok(vm.get_return_code())
    }

    // Extract preprocessor directives from source code
    // Directives start with '/' and are removed from the main code
    fn extract_directives(&self, source: &str) -> Result<(String, Vec<PreprocessorDirective>)> {
        let mut result = String::new();
        let mut directives = Vec::new();
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check for preprocessor directive
            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
                let start = i;
                let mut j = i + 1;
                // Collect the entire directive including parameters
                while j < chars.len()
                    && (chars[j].is_ascii_digit()
                    || chars[j] == ' '
                    || chars[j] == '0'
                    || chars[j] == '1')
                    {
                        j += 1;
                    }
                    let directive_str: String = chars[start..j].iter().collect();
                if let Ok(directive) = PreprocessorDirective::parse(&directive_str) {
                    directives.push(directive);
                    i = j;
                    continue;
                }
                // If not a valid directive, treat as normal code
                result.push(chars[i]);
                i += 1;
                continue;
            }

            // Skip comments < comment content >
            if chars[i] == '<' {
                let mut depth = 1;
                i += 1;
                while i < chars.len() && depth > 0 {
                    if chars[i] == '<' {
                        depth += 1;
                    } else if chars[i] == '>' {
                        depth -= 1;
                    }
                    i += 1;
                }
                continue;
            }

            result.push(chars[i]);
            i += 1;
        }

        Ok((result, directives))
    }

    fn execute_directive(&mut self, directive: &PreprocessorDirective) -> Result<()> {
        match directive {
            PreprocessorDirective::ReplaceChars(zero, one) => {
                self.char_zero = *zero;
                self.char_one = *one;
                Ok(())
            }
            PreprocessorDirective::SetBitWidth(width) => {
                if *width < 2 {
                    return Err(anyhow!("Bit width must be at least 2"));
                }
                self.bit_width = *width;
                Ok(())
            }
            PreprocessorDirective::Reserved => Ok(()),
        }
    }

    // Clean source: remove comments, whitespace, convert custom chars to 0/1
    fn clean_source(&self, source: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            // Skip comments delimited by < and >
            if ch == '<' {
                let mut depth = 1;
                i += 1;
                while i < chars.len() && depth > 0 {
                    if chars[i] == '<' {
                        depth += 1;
                    } else if chars[i] == '>' {
                        depth -= 1;
                    }
                    i += 1;
                }
                continue;
            }

            // Skip whitespace
            if ch.is_whitespace() {
                i += 1;
                continue;
            }

            // Convert custom characters to 0/1, or keep 0/1 as-is
            if ch == self.char_zero {
                result.push('0');
            } else if ch == self.char_one {
                result.push('1');
            } else if ch == '0' || ch == '1' {
                result.push(ch);
            }
            // Skip any other characters (they are comments or invalid)

            i += 1;
        }

        result
    }

    // Parse cleaned binary string into instructions
    fn parse_binary(&mut self, binary: &str) -> Result<(Vec<Instruction>, HashMap<u64, usize>)> {
        let mut instructions = Vec::new();
        let mut labels = HashMap::new();
        let chars: Vec<char> = binary.chars().collect();
        let mut i = 0;
        let bw = self.bit_width as usize;

        while i < chars.len() {
            // Need at least 3 bits for opcode
            if i + 3 > chars.len() {
                if i < chars.len() {
                    eprintln!("Warning: {} leftover bits ignored", chars.len() - i);
                }
                break;
            }

            // Read opcode (3 bits)
            let opcode = self.read_bits(&chars, &mut i, 3)?;

            match opcode {
                0 => {
                    // SET: 000 register parameter mode
                    // Length: 3 + bw + bw + 1
                    if i + bw + bw + 1 > chars.len() {
                        return Err(anyhow!("Incomplete SET instruction at bit {}", i));
                    }
                    let reg = self.read_bits(&chars, &mut i, bw)?;
                    let param = self.read_bits(&chars, &mut i, bw)?;
                    let mode = self.read_bits(&chars, &mut i, 1)?;
                    instructions.push(Instruction::Set {
                        reg,
                        param,
                        mode: mode as u8,
                    });
                }
                1 => {
                    // ADD: 001 registerA parameter mode
                    if i + bw + bw + 1 > chars.len() {
                        return Err(anyhow!("Incomplete ADD instruction at bit {}", i));
                    }
                    let reg_a = self.read_bits(&chars, &mut i, bw)?;
                    let param = self.read_bits(&chars, &mut i, bw)?;
                    let mode = self.read_bits(&chars, &mut i, 1)?;
                    instructions.push(Instruction::Add {
                        reg_a,
                        param,
                        mode: mode as u8,
                    });
                }
                2 => {
                    // SUB: 010 registerA parameter mode
                    if i + bw + bw + 1 > chars.len() {
                        return Err(anyhow!("Incomplete SUB instruction at bit {}", i));
                    }
                    let reg_a = self.read_bits(&chars, &mut i, bw)?;
                    let param = self.read_bits(&chars, &mut i, bw)?;
                    let mode = self.read_bits(&chars, &mut i, 1)?;
                    instructions.push(Instruction::Sub {
                        reg_a,
                        param,
                        mode: mode as u8,
                    });
                }

                3 => {
                    // JMP: 011 parameter mode1 mode2
                    if i + bw + 1 + 1 > chars.len() {
                        return Err(anyhow!("Incomplete JMP instruction at bit {}", i));
                    }
                    let param = self.read_bits(&chars, &mut i, bw)?;
                    let mode1 = self.read_bits(&chars, &mut i, 1)?;
                    let mode2 = self.read_bits(&chars, &mut i, 1)?;
                    let def = mode1 == 1;
                    let use_register = mode2 == 1;

                    // Record label definition
                    if def {
                        if labels.contains_key(&param) {
                            return Err(anyhow!("Label {} already defined", param));
                        }
                        labels.insert(param, instructions.len());
                    }

                    instructions.push(Instruction::Jmp {
                        param,
                        def,
                        use_register,
                    });
                }

                4 => {
                    // IFZ: 100 register parameter mode
                    if i + bw + bw + 1 > chars.len() {
                        return Err(anyhow!("Incomplete IFZ instruction at bit {}", i));
                    }
                    let reg = self.read_bits(&chars, &mut i, bw)?;
                    let param = self.read_bits(&chars, &mut i, bw)?;
                    let mode = self.read_bits(&chars, &mut i, 1)?;
                    instructions.push(Instruction::Ifz {
                        reg,
                        param,
                        mode: mode as u8,
                    });
                }
                5 => {
                    // OUT: 101 parameter mode1 mode2
                    if i + bw + 2 + 1 > chars.len() {
                        return Err(anyhow!("Incomplete OUT instruction at bit {}", i));
                    }
                    let param = self.read_bits(&chars, &mut i, bw)?;
                    let mode1 = self.read_bits(&chars, &mut i, 2)?;
                    let mode2 = self.read_bits(&chars, &mut i, 1)?;
                    instructions.push(Instruction::Out {
                        param,
                        mode1: mode1 as u8,
                        mode2: mode2 as u8,
                    });
                }
                6 => {
                    // INP: 110 register mode
                    if i + bw + 2 > chars.len() {
                        return Err(anyhow!("Incomplete INP instruction at bit {}", i));
                    }
                    let reg = self.read_bits(&chars, &mut i, bw)?;
                    let mode = self.read_bits(&chars, &mut i, 2)?;
                    instructions.push(Instruction::Inp {
                        reg,
                        mode: mode as u8,
                    });
                }
                7 => {
                    // SYS: 111 parameter mode
                    if i + bw + 1 > chars.len() {
                        return Err(anyhow!("Incomplete SYS instruction at bit {}", i));
                    }
                    let param = self.read_bits(&chars, &mut i, bw)?;
                    let mode = self.read_bits(&chars, &mut i, 1)?;
                    instructions.push(Instruction::Sys {
                        param,
                        mode: mode as u8,
                    });
                }
                _ => return Err(anyhow!("Unknown opcode: {}", opcode)),
            }
        }

        Ok((instructions, labels))
    }

    // Read a specified number of bits from the character array
    fn read_bits(&self, chars: &[char], pos: &mut usize, count: usize) -> Result<u64> {
        if *pos + count > chars.len() {
            return Err(anyhow!("Not enough bits remaining at position {}", *pos));
        }

        let mut value = 0u64;
        for _ in 0..count {
            let ch = chars[*pos];
            if ch == '1' {
                value = (value << 1) | 1;
            } else if ch == '0' {
                value <<= 1;
            } else {
                return Err(anyhow!(
                    "Invalid binary character: '{}' at position {}",
                    ch,
                    *pos
                ));
            }
            *pos += 1;
        }
        Ok(value)
    }
}
