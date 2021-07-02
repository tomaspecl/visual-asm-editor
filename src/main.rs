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
#![allow(non_snake_case)]

mod codeblock;
mod parser;
mod linker;
mod splitter;
mod util;
mod codeblockholder;
mod textboxholder;
mod codeblockwindow;

use codeblock::*;

use druid::*;
use druid::widget::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::codeblockwindow::CodeBlockWindow;

#[derive(Clone, Data, Lens)]
struct MyData {
	code: CodeBlocks,
	mouse_click_pos: Option<Point>,
	mouse_pos: Point,
	drag_mode: bool,
}

fn print_code(code: &Vec<Rc<RefCell<CodeBlock>>>) {
	print!("code = [\n\n");
	for part in code {
		println!("Rc({:p}) {:?}\n",*part,**part);
	}
	println!("]");
}

fn main() {
	let _text = "\
	;#codeblock,0,200,\n\
	;add array elements [bytes]\n\
	;start of array in ebx;length in ecx;sum in eax \n\
	myfunc:\n\
	add ecx, ebx\n\
	xor eax, eax\n\
	;#codeblock,160,80,\n\
	loop:\n\
	cmp ebx, ecx\n\
	je end\n\
	;#codeblock,260,20,\n\
	movzx edx, byte [ebc]\n\
	add eax, edx\n\
	inc ebx\n\
	jmp loop\n\
	;#codeblock,160,300,\n\
	end:\n\
	ret\
	";

	let _text2 = "\
	;#codeblock,0,100,\n\
	;add array elements [bytes]\n\
	;start of array in ebx;length in ecx;sum in eax \n\
	myfunc:\n\
	add ecx, ebx\n\
	xor eax, eax\n\
	;#codeblock,80,40,\n\
	loop:\n\
	cmp ebx, ecx\n\
	je end\n\
	movzx edx, byte [ebc]\n\
	add eax, edx\n\
	inc ebx\n\
	jmp loop\n\
	;#codeblock,80,150,\n\
	end:\n\
	ret\
	";

	let _text3 = "\
	;add array elements [bytes]\n\
	;start of array in ebx;length in ecx;sum in eax \n\
	myfunc:\n\
	add ecx, ebx\n\
	xor eax, eax\n\
	loop:\n\
	cmp ebx, ecx\n\
	je end\n\
	movzx edx, byte [ebc]\n\
	add eax, edx\n\
	inc ebx\n\
	jmp loop\n\
	end:\n\
	ret\
	";

	let _text = "\
	;#codeblock,100,100,\n\
	something
	";

	let text = "";

	//let text3 = std::fs::read_to_string("asm.txt").unwrap();

	let mut data = parser::parse(&text);
	splitter::split(&mut data);
	linker::link(&data);

	print_code(&data);

	let data = MyData{ code: CodeBlocks::new(data), mouse_click_pos: None, mouse_pos: Point::new(0.0,0.0), drag_mode: false};
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window)/*.log_to_console()*/.launch(data).expect("launch failed");
}

fn ui_builder() -> impl Widget<MyData> {
	let button = Button::new("print").on_click(|_ctx, data: &mut MyData, _env| print_code(&data.code.borrow())).padding(5.0);

	let codeblockwindow = CodeBlockWindow::new();

	Flex::column().with_child(button).with_flex_child(Padding::new(10.0, codeblockwindow),1.0).debug_paint_layout()

}
