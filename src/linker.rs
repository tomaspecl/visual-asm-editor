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
use crate::codeblock::*;
use crate::util::*;

use std::rc::Rc;
use std::cell::RefCell;

/*

Every codeblock has max one jump and max one conditional jump
Every codeblock has max one label which is at the top

 */

pub fn link(code: &Vec<Rc<RefCell<CodeBlock>>>) {
    let labels = {
        let mut labels = Vec::new();
        for block in code {
            let mut label_vec = block.borrow().label();
            if let Some(label) = label_vec.pop() {
                assert!(label_vec.is_empty());
                labels.push((label.label,Rc::downgrade(block)));
            }
        }
        labels
    };

    for block in code {
        let mut block = block.borrow_mut();
        let mut jump = None;
        let mut jump_cond = None;
        let mut jump_cond_line = 0;
        let mut jump_cond_line_offset = 0;
        for (i, line) in block.text.lines().enumerate() {
            let line_offset = line.as_ptr() as usize - block.text.as_ptr() as usize;
            match contains_jump(line) {
                JumpType::None => {},
                JumpType::Jmp(label) => {
                    if jump.is_none() {
                        jump = Some(label);
                    }else{
                        panic!("jump already set");
                    }
                },
                JumpType::CondJmp(label) => {
                    if jump_cond.is_none() {
                        jump_cond = Some(label);
                        jump_cond_line = i;
                        jump_cond_line_offset = line_offset;
                    }else{
                        panic!("jump_cond already set");
                    }
                },
            }
        }

        if let Some(label) = jump {
            if let Some(block_ref) = labels.iter().find(|e| e.0 == label) {
                block.next = block_ref.clone().1;
            }
        }

        if let Some(label) = jump_cond {
            if let Some(block_ref) = labels.iter().find(|e| e.0 == label) {
                block.next_branch = block_ref.clone().1;
                block.next_branch_line = jump_cond_line;
                block.next_branch_line_offset = jump_cond_line_offset;
            }
        }
    }
}