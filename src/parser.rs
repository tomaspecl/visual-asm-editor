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
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use druid::kurbo::Point;

pub fn parse<'a>(str: &'a str) -> Result<Vec<Rc<RefCell<CodeBlock>>>,ParseError<'a>> {
    let mut code = Vec::new();

    code.push(Rc::new(RefCell::new(CodeBlock::default())));

    for (i,line) in str.lines().enumerate() {
        if line.starts_with(";#") {
            //visual asm editor metadata
            //format: ;#parameter1,parameter2,parameter3, ... ,
            let parameters = line[2..].split(',').collect::<Vec<&str>>();

            match parameters[0] {
                "codeblock" => {    //start of codeblock => push new codeblock on code vector
                    if parameters.len()<3 {
                        return Err(ParseError{
                            desc: "Not enough parameters",
                            line: i,
                            text: line,
                        });
                    }
                    assert!(parameters.len()>=3);
                    let x = parameters[1].parse::<f64>().map_err(|_| return ParseError{
                        desc: "Not a valid number parameter",
                        line: i,
                        text: line,
                    })?;
                    let y = parameters[2].parse::<f64>().map_err(|_| return ParseError{
                        desc: "Not a valid number parameter",
                        line: i,
                        text: line,
                    })?;
                    let new_codeblock = Rc::new(RefCell::new(CodeBlock {
                        pos: Point { x, y },
                        ..Default::default()
                    }));

                    {
                        let mut prev_codeblock = code.last().expect("there should be at least one codeblock").borrow_mut();

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
                    return Err(ParseError{
                        desc: "Unknown metadata type",
                        line: i,
                        text: line,
                    });
                }
            }

        }else{
            //normal text => append to last codeblock
            let mut codeblock = code.last().expect("there should be at least one codeblock").borrow_mut();
            if !codeblock.text.is_empty() {
                codeblock.text.push('\n');
            }
            codeblock.text.push_str(line);
        }
    }

    Ok(code)
}

#[derive(Debug)]
pub struct ParseError<'a> {
    desc: &'static str,
    line: usize,
    text: &'a str,
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"File can not be parsed: {}\nError occured on line {}:\n{}",self.desc,self.line+1,self.text)
    }
}

impl<'a> Error for ParseError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}