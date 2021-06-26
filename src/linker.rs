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
        for line in block.text.lines() {
            match contains_jump(line) {
                JumpType::None => {},
                JumpType::Jmp(label) => {
                    if jump.is_none() {
                        jump = Some(label)
                    }else{
                        panic!("jump already set");
                    }
                },
                JumpType::CondJmp(label) => {
                    if jump_cond.is_none() {
                        jump_cond = Some(label)
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
            }
        }
    }
}