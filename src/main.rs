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
use std::io::Write;
use std::path::PathBuf;
use crate::codeblockwindow::CodeBlockWindow;

#[derive(Clone, Data, Lens)]
struct MyData {
    code: CodeBlocks,
    #[data(same_fn = "PartialEq::eq")]
    current_file: Option<PathBuf>,
    mouse_click_pos: Option<Point>,
    mouse_pos: Point,
    drag_mode: bool,
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

    let _text1 = "\
    ;#codeblock,508,11,\n\
    ;add array elements [bytes]\n\
    ;start of array in ebx;length in ecx;sum in eax\n\
    ;#codeblock,591,152,\n\
    myfunc:\n\
    add ecx, ebx\n\
    xor eax, eax\n\
    ;#codeblock,470,313,\n\
    loop:\n\
    cmp ebx, ecx\n\
    je end\n\
    ;#codeblock,191,434,\n\
    movzx edx, byte [ebc]\n\
    add eax, edx\n\
    inc ebx\n\
    jmp loop\n\
    ;#codeblock,831,497,\n\
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

    let _text4 = "\
    ;#codeblock,100,100,\n\
    something
    ";

    let mut data = parser::parse(&_text1).unwrap();
    splitter::split(&mut data);
    linker::link(&data);

    let code = CodeBlocks::new(data);

    dbg!(&code);

    let data = MyData{ code, current_file: None, mouse_click_pos: None, mouse_pos: Point::new(0.0,0.0), drag_mode: false };

    let main_window = WindowDesc::new(ui_builder()).menu(menu_builder);
    AppLauncher::with_window(main_window)/*.log_to_console()*/.launch(data).expect("launch failed");
}

fn menu_builder(_winid: Option<WindowId>, _data: &MyData, _env: &Env) -> Menu<MyData> {
    use druid::menu::sys::win::file;
    //let file = file::default::<MyData>();

    let file = Menu::new(LocalizedString::new("common-menu-file-menu"))
    .entry(file::open())
    .entry(file::save())
    .entry(file::save_as())
    .separator()
    .entry(file::exit());

    druid::menu::Menu::new("File").entry(file)
}

fn ui_builder() -> impl Widget<MyData> {
    let button = Button::new("print debug").on_click(|_ctx, data: &mut MyData, _env| {dbg!(&data.code);}).padding(5.0);
    let button2 = Button::new("print").on_click(|_ctx, data: &mut MyData, _env| println!("{}",data.code) ).padding(5.0);

    let codeblockwindow = CodeBlockWindow::new();

    let command_handler = CommandHandler;

    Flex::column().with_child(Flex::row().with_child(button).with_child(button2)).with_flex_child(Padding::new(10.0, codeblockwindow),1.0).debug_paint_layout().controller(command_handler)

}

struct CommandHandler;

impl<W: Widget<MyData>> Controller<MyData, W> for CommandHandler {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut MyData, env: &Env) {
        match event {
            Event::WindowCloseRequested => (),
            Event::WindowDisconnected => (),
            Event::Paste(_) => (),
            Event::Zoom(_) => (),
            Event::Timer(_) => (),
            Event::Notification(_) => (),
            Event::Command(cmd) => {
                // TODO: display nice error messages
                if let Some(file) = cmd.get(commands::OPEN_FILE) {
                    // TODO: warn when current file is not saved
                    let path = file.path();
                    match std::fs::read_to_string(path) {
                        Ok(text) => {
                            data.current_file = Some(path.to_path_buf());

                            match parser::parse(&text) {
                                Ok(mut new_data) => {
                                    splitter::split(&mut new_data);
                                    linker::link(&new_data);
                                    let code = CodeBlocks::new(new_data);
                                    data.code = code;

                                    let selector = druid::Selector::new("reload");
                                    let command = druid::Command::new(selector, (), druid::Target::Global);
                                    ctx.submit_command(command);
                                    return;
                                },
                                Err(e) => println!("Could not read file: {}", e),
                            }
                            
                        },
                        Err(e) => println!("Could not read file: {}", e),
                    }
                }else if let Some(file) = cmd.get(commands::SAVE_FILE_AS) {
                    let path = file.path();

                    let file = std::fs::OpenOptions::new().write(true).create_new(true).open(path);

                    match file {
                        Ok(mut file) => {
                            data.current_file = Some(path.to_path_buf());

                            let text = data.code.to_string();
                            if let Err(e) = file.write_all(text.as_bytes()) {
                                println!("Could not write to file: {}",e);
                            }
                        },
                        Err(e) => println!("Could not create file: {}",e),
                    }
                }else if let Some(()) = cmd.get(commands::SAVE_FILE) {
                    if let Some(pathbuf) = &data.current_file {
                        let file = std::fs::OpenOptions::new().write(true).open(pathbuf);

                        match file {
                            Ok(mut file) => {
                                let text = data.code.to_string();
                                if let Err(e) = file.write_all(text.as_bytes()) {
                                    println!("Could not write to file: {}",e);
                                }
                            },
                            Err(e) => println!("Could not create file: {}",e),
                        }
                    }else{
                        ctx.submit_command(commands::SHOW_SAVE_PANEL.with(FileDialogOptions::default()));
                    }
                }
            },
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}
