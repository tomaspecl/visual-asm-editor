/*
Visual asm editor
Copyright (C) 2021  Tomáš Pecl

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
const JUMPS: &[&str] = &["jmp"];
const CONDITIONAL_JUMPS: &[&str] = &["ja", "jae", "jb", "jb", "jbe", "jc", "jcxz", "jecxz", "jrcxz", "je", "jg", "jge", "jl", "jle", "jna", "jnae", "jnb", "jnbe", "jnc", "jne", "jng", "jnge", "jnl", "jnle", "jno", "jnp", "jns", "jnz", "jo", "jp", "jpe", "jpo", "js", "jz"];

// Optional(white-space) label: Optional(white-space) instruction operands Optional(white-space) Optional(;comment)
pub fn contains_jump(str: &str) -> JumpType {
    let instruction_operands = str.trim_start().split(';').next().unwrap().trim_end().rsplit(':').next().unwrap().trim_start();

    let mut i = instruction_operands.split_whitespace();
    if let Some(jmp) = i.next() {
        if let Some(label) = i.next() {
            if JUMPS.contains(&&jmp.to_lowercase()[..]) {
                JumpType::Jmp(label.to_string())
            }else if CONDITIONAL_JUMPS.contains(&&jmp.to_lowercase()[..]) {
                JumpType::CondJmp(label.to_string())
            }else{
                JumpType::None
            }
        }else{
            JumpType::None
        }
    }else{
        JumpType::None
    }
}

pub enum JumpType {
    None,
    Jmp(String),
    CondJmp(String),
}