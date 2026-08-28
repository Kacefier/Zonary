//! Zonary Interpreter - Made by Kacefier - Version 2.1.1 - Virtual Machine
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
use std::io::{self, Write};

use anyhow::{anyhow, Result};
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

pub const DEFAULT_BIT_WIDTH: u8 = 8;

#[derive(Debug, Clone)]
pub enum Instruction {
    Set { reg: u64, param: u64, mode: u8 },
    Add { reg_a: u64, param: u64, mode: u8 },
    Sub { reg_a: u64, param: u64, mode: u8 },
    Jmp { param: u64, def: bool, use_register: bool },
    Ifz { reg: u64, param: u64, mode: u8 },
    Out { param: u64, mode1: u8, mode2: u8 },
    Inp { reg: u64, mode: u8 },
    Sys { param: u64, mode: u8 },
}

pub struct VM {
    bit_width: u8,
    registers: HashMap<u64, BigUint>,
    pc: usize,
    return_code: i32,
}

impl VM {
    pub fn new(bit_width: u8) -> Self {
        Self {
            bit_width,
            registers: HashMap::new(),
            pc: 0,
            return_code: 0,
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction], labels: &HashMap<u64, usize>) -> Result<()> {
        self.pc = 0;
        let mut exited = false;

        while self.pc < instructions.len() {
            let instruction = &instructions[self.pc];

            if self.execute_instruction(instruction, instructions, labels)? {
                exited = true;
                break;
            }

            self.pc += 1;
        }

        // Only print default return code if program didn't exit via SYS 0
        if !exited {
            let zeros = self.format_binary(&BigUint::zero());
            println!("\n---\n[Return Code: {}]", zeros);
        }

        Ok(())
    }

    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
        _instructions: &[Instruction],
        labels: &HashMap<u64, usize>,
    ) -> Result<bool> {
        match instruction {
            Instruction::Set { reg, param, mode } => {
                let value = if *mode == 0 {
                    self.to_biguint(*param)
                } else {
                    self.get_register(*param)
                };
                self.set_register(*reg, value);
                Ok(false)
            }
            Instruction::Add { reg_a, param, mode } => {
                let a = self.get_register(*reg_a);
                let b = if *mode == 0 {
                    self.to_biguint(*param)
                } else {
                    self.get_register(*param)
                };
                let result = a + b;
                self.set_register(*reg_a, result);
                Ok(false)
            }
            Instruction::Sub { reg_a, param, mode } => {
                let a = self.get_register(*reg_a);
                let b = if *mode == 0 {
                    self.to_biguint(*param)
                } else {
                    self.get_register(*param)
                };
                let max = self.get_max_value();
                let result = if a < b {
                    (max - b) + a + BigUint::one()
                } else {
                    a - b
                };
                self.set_register(*reg_a, result);
                Ok(false)
            }
            Instruction::Jmp { param, def, use_register } => {
                let label_num = if *use_register {
                    self.get_register(*param).to_u64_digits().first().map(|&x| x).unwrap_or(0)
                } else {
                    *param
                };

                if *def {
                    Ok(false)
                } else {
                    if let Some(&target_pc) = labels.get(&label_num) {
                        self.pc = target_pc;
                        Ok(false)
                    } else {
                        eprintln!("Warning: Label {} not found, jumping ignored", label_num);
                        Ok(false)
                    }
                }
            }
            Instruction::Ifz { reg, param, mode } => {
                let value = self.get_register(*reg);
                if value.is_zero() {
                    let label_num = if *mode == 0 {
                        *param
                    } else {
                        self.get_register(*param).to_u64_digits().first().map(|&x| x).unwrap_or(0)
                    };
                    if let Some(&target_pc) = labels.get(&label_num) {
                        self.pc = target_pc;
                    } else {
                        eprintln!("Warning: Label {} not found, jumping ignored", label_num);
                    }
                }
                Ok(false)
            }
            Instruction::Out { param, mode1, mode2 } => {
                let value = if *mode2 == 0 {
                    self.to_biguint(*param)
                } else {
                    self.get_register(*param)
                };
                self.output_value(&value, *mode1)?;
                Ok(false)
            }
            Instruction::Inp { reg, mode } => {
                let value = self.read_input(*mode)?;
                self.set_register(*reg, value);
                Ok(false)
            }
            Instruction::Sys { param, mode } => {
                if *mode == 0 {
                    // Terminate program and print return code
                    self.return_code = *param as i32;
                    let code = self.to_biguint(*param);
                    println!("\n---\n[Return Code: {}]", self.format_binary(&code));
                    Ok(true)
                } else {
                    // Delay execution
                    let ms = *param;
                    if ms > 0 {
                        std::thread::sleep(std::time::Duration::from_millis(ms));
                    }
                    Ok(false)
                }
            }
        }
    }

    fn get_register(&self, reg: u64) -> BigUint {
        self.registers.get(&reg).cloned().unwrap_or(BigUint::zero())
    }

    fn set_register(&mut self, reg: u64, value: BigUint) {
        let masked = self.truncate(value);
        if masked.is_zero() {
            self.registers.remove(&reg);
        } else {
            self.registers.insert(reg, masked);
        }
    }

    fn get_max_value(&self) -> BigUint {
        if self.bit_width == 0 {
            return BigUint::zero();
        }
        (BigUint::one() << (self.bit_width as usize)) - BigUint::one()
    }

    fn truncate(&self, value: BigUint) -> BigUint {
        let max = self.get_max_value();
        if value > max {
            value & max
        } else {
            value
        }
    }

    fn to_biguint(&self, value: u64) -> BigUint {
        value.to_biguint().unwrap_or(BigUint::zero())
    }

    fn format_binary(&self, value: &BigUint) -> String {
        let bits = self.bit_width as usize;
        let val = self.truncate(value.clone());
        let bin_str = val.to_str_radix(2);
        if bin_str.len() < bits {
            format!("{:0>width$}", bin_str, width = bits)
        } else {
            bin_str
        }
    }

    fn output_value(&self, value: &BigUint, mode: u8) -> Result<()> {
        let val = self.truncate(value.clone());
        match mode {
            0 => {
                print!("{}", self.format_binary(&val));
            }
            1 => {
                print!("{}", val);
            }
            2 => {
                let digits = (self.bit_width as usize + 3) / 4;
                let hex_str = val.to_str_radix(16).to_uppercase();
                if hex_str.len() < digits {
                    print!("{:0>width$}", hex_str, width = digits);
                } else {
                    print!("{}", hex_str);
                }
            }
            3 => {
                let val_u64 = val.to_u64_digits().first().map(|&x| x).unwrap_or(0);
                if (32..=126).contains(&val_u64) {
                    print!("{}", val_u64 as u8 as char);
                } else {
                    print!("\\x{:02X}", val_u64 & 0xFF);
                }
            }
            _ => return Err(anyhow!("Invalid output mode: {}", mode)),
        }
        io::stdout().flush()?;
        Ok(())
    }

    fn read_input(&self, mode: u8) -> Result<BigUint> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer = buffer.trim();

        if buffer.is_empty() {
            return Ok(BigUint::zero());
        }

        let result = match mode {
            0 => {
                if buffer.chars().all(|c| c == '0' || c == '1') {
                    let bits = self.bit_width as usize;
                    if buffer.len() == bits {
                        BigUint::parse_bytes(buffer.as_bytes(), 2)
                        .unwrap_or(BigUint::zero())
                    } else {
                        BigUint::zero()
                    }
                } else {
                    BigUint::zero()
                }
            }
            1 => {
                BigUint::parse_bytes(buffer.as_bytes(), 10)
                .unwrap_or(BigUint::zero())
            }
            2 => {
                let expected_digits = (self.bit_width as usize + 3) / 4;
                let clean = buffer.trim_start_matches('0');
                if clean.is_empty() || clean.len() <= expected_digits {
                    if buffer.chars().all(|c| c.is_ascii_hexdigit()) {
                        BigUint::parse_bytes(buffer.as_bytes(), 16)
                        .unwrap_or(BigUint::zero())
                    } else {
                        BigUint::zero()
                    }
                } else {
                    BigUint::zero()
                }
            }
            3 => {
                if let Some(c) = buffer.chars().next() {
                    let code = c as u64;
                    code.to_biguint().unwrap_or(BigUint::zero())
                } else {
                    BigUint::zero()
                }
            }
            _ => return Err(anyhow!("Invalid input mode: {}", mode)),
        };

        Ok(self.truncate(result))
    }

    pub fn get_return_code(&self) -> i32 {
        self.return_code
    }
}
