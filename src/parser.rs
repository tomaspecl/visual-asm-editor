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
use crate::codeblock::CodeBlock;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use druid::kurbo::Point;

pub fn parse(str: &str) -> Vec<Rc<RefCell<CodeBlock>>> {

    let mut code = Vec::new();

    code.push(Rc::new(RefCell::new(CodeBlock{
        pos: Point::new(0.0,0.0),
        size: Default::default(),
        text: String::new(),
        next: Weak::new(),
        next_branch: Weak::new(),
        next_branch_line: 0,
    })));

    for line in str.lines() {
        if line.starts_with(";#") {
            //visual asm editor metadata
            //format: ;#parameter1,parameter2,parameter3, ... ,
            let parameters = line[2..].split(',').collect::<Vec<&str>>();

            match parameters[0] {
                "codeblock" => {    //start of codeblock => push new codeblock on code vector
                    let new_codeblock = Rc::new(RefCell::new(CodeBlock {
                        pos: Point { x: parameters[1].parse::<f64>().unwrap(), y: parameters[2].parse::<f64>().unwrap() },
                        size: Default::default(),
                        text: String::new(),
                        next: Weak::new(),
                        next_branch: Weak::new(),
                        next_branch_line: 0,
                    }));

                    {
                        let mut prev_codeblock = code.last().unwrap().borrow_mut();

                        if prev_codeblock.text.is_empty() {
                            drop(prev_codeblock);
                            let _ = code.pop();
                        } else {//link new codeblock as next of previous codeblock
                            prev_codeblock.next = Rc::downgrade(&new_codeblock);
                        }
                    }

                    code.push(new_codeblock);
                }
                _ => {
                    //other
                    todo!();
                }
            }

        }else{
            //normal text => append to last codeblock
            let mut codeblock = code.last().unwrap().borrow_mut();
            if !codeblock.text.is_empty() {
                codeblock.text.push('\n');
            }
            codeblock.text.push_str(line);
        }
    }

    code
}