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

use druid::kurbo::Vec2;
use std::rc::Rc;
use std::cell::RefCell;

pub fn split(code: &mut Vec<Rc<RefCell<CodeBlock>>>) {
    split_labels(code);
    split_jumps(code);
}

fn split_labels(code: &mut Vec<Rc<RefCell<CodeBlock>>>) {
    let mut new_blocks = Vec::new();

    for block in code.iter() {
        let mut block = block.borrow_mut();
        let mut labels = block.label();

        while labels.len()>1 {
            let new_string = block.text.split_off(labels.pop().unwrap().offset);
            let new_block = Rc::new(RefCell::new(CodeBlock {
                pos: block.pos+Vec2::new(0.0,20.0), //TODO: better positioning
                text: new_string,
                next: block.next.clone(),
                ..Default::default()
            }));

            block.next=Rc::downgrade(&new_block);
            new_blocks.push(new_block);
        }
        block.text.shrink_to_fit();
    }
    code.append(&mut new_blocks);
}

fn split_jumps(code: &mut Vec<Rc<RefCell<CodeBlock>>>) {
    let mut new_blocks = Vec::new();

    for block in code.iter() {
        let mut block = block.borrow_mut();

        let mut splits = Vec::new();
        let mut jump = false;
        let mut jump_cond = false;
        for line in block.text.lines() {
            if jump==true {
                //when previous line == jump => split
                splits.push(line.as_ptr() as usize - block.text.as_ptr() as usize);
                jump=false;
                jump_cond=false;
                continue;
            }
            match contains_jump(line) {
                JumpType::Jmp(_) => {
                    assert!(!jump, "should not happen");
                    jump = true;
                },
                JumpType::CondJmp(_) => {
                    if !jump_cond {
                        jump_cond = true;
                    }else{
                        //when previous line == cond_jump => split
                        splits.push(line.as_ptr() as usize - block.text.as_ptr() as usize);
                        jump=false;
                        //jump_cond=true;
                    }
                },
                JumpType::None => (),
            }
        }

        for &split in splits.iter().rev() {
            let new_string = block.text.split_off(split);
            let new_block = Rc::new(RefCell::new(CodeBlock {
                pos: block.pos+Vec2::new(0.0,20.0), //TODO: better positioning
                text: new_string,
                next: block.next.clone(),
                ..Default::default()
            }));

            block.next=Rc::downgrade(&new_block);
            new_blocks.push(new_block);
        }
        block.text.shrink_to_fit();

    }

    code.append(&mut new_blocks);
}