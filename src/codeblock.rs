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
use std::cell::{RefCell, Ref, RefMut, Cell};
use std::rc::{Weak, Rc};
use druid::kurbo::{Point, Size};
use std::fmt::{Debug, Formatter};
use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct CodeBlock {
    pub pos: Point,
    pub size: Size,
    pub text: String,
    pub next: Weak<RefCell<CodeBlock>>,
    pub next_branch: Weak<RefCell<CodeBlock>>,
    pub next_branch_line: usize,
    pub next_branch_line_offset: usize,
}

impl Default for CodeBlock {
    fn default() -> Self {
        CodeBlock {
            pos: Point::default(),
            size: Size::default(),
            text: String::default(),
            next: Weak::default(),
            next_branch: Weak::default(),
            next_branch_line: 0,
            next_branch_line_offset: 0
        }
    }
}

pub struct CodeBlocks {
    pub changed: Cell<u64>,
    vec: Rc<RefCell<Vec<Rc<RefCell<CodeBlock>>>>>,
}

impl Clone for CodeBlocks {
    fn clone(&self) -> Self {
        CodeBlocks{changed: self.changed.clone(), vec: self.vec.clone()}
    }
}

// TODO: is it even helping?
impl Data for CodeBlocks {
    fn same(&self, other: &Self) -> bool {
        //println!("called same() on CodeBlocks - changed={} other={}", self.changed.get(), other.changed.get());
        self.changed.get() == other.changed.get()
    }
}

impl CodeBlocks {
    pub fn new(vec: Vec<Rc<RefCell<CodeBlock>>>) -> Self { CodeBlocks{changed: Cell::new(0), vec: Rc::new(RefCell::new(vec))} }
    pub fn borrow(&self) -> Ref<Vec<Rc<RefCell<CodeBlock>>>> { self.vec.borrow() }
    pub fn borrow_mut(&self) -> RefMut<Vec<Rc<RefCell<CodeBlock>>>> { self.changed.set(self.changed.get()+1); self.vec.borrow_mut() }
}

pub struct Label {
    pub label: String,
    pub offset: usize,
    //pub offset_end: usize,
}

impl CodeBlock {
    pub fn label(&self) -> Vec<Label> {
        let mut labels = Vec::new();
        for line in self.text.lines() {
            if let Some(label) = line.trim_start().split_inclusive(|c| c==';' || c==':' || char::is_whitespace(c)).next().and_then(|x| x.strip_suffix(':')) {
                let label_offset_line = line.as_ptr() as usize - self.text.as_ptr() as usize;
                let label_offset = label.as_ptr() as usize - self.text.as_ptr() as usize;
                labels.push(Label{
                    label: String::from(&self.text[label_offset..label_offset + label.len()]),
                    offset: label_offset_line,
                    //offset_end: label_offset + label.len(),
                });
                //println!("{}",&self.text[label_offset..label_offset + label.len()]);
            }
        }
        labels
    }
}

impl Debug for CodeBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"CodeBlock {{\n\t\
                pos: {:?}\n\t\
                text: {:?}\n\t\
                next: Weak({:?})\n\t\
                next_branch: Weak({:?})\n\
            }}",
            self.pos,
            self.text,
            self.next.as_ptr(),
            self.next_branch.as_ptr(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn label_works() {
        let str="\
        fff:;ffffad\n\
        label :dsffsd\n\
        label2:\n\
        label3:\n\
        somecode;asd:asf\n\
        ff\n\
        ";

        let c = CodeBlock {
            text: str.to_string(),
            ..Default::default()
        };

        let mut vec = c.label();

        assert!(vec.drain(..).map(|e| e.label).collect::<Vec<String>>() ==vec!["fff","label2","label3"]);

    }
}