#![allow(non_snake_case)]

mod codeblock;
mod parser;
mod linker;
mod splitter;
mod util;
mod interceptor;
mod codeblockholder;
mod codeblockwindow;
mod clip_box;

use codeblock::*;
use interceptor::Interceptor;

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

/*impl Data for Rc<Vec<Rc<RefCell<CodeBlock>>>> {
	fn same(&self, _: &Self) -> bool {

	}
}*/

fn print_code(code: &Vec<Rc<RefCell<CodeBlock>>>) {
	print!("code = [\n\n");
	for part in code {
		println!("Rc({:p}) {:?}\n",*part,**part);
	}
	println!("]");
}

fn main() {
/*
	let _text = "\
	;#codeblock,0,10,\n\
	;add array elements [bytes]\n\
	;start of array in ebx;length in ecx;sum in eax \n\
	myfunc:\n\
	add ecx, ebx\n\
	xor eax, eax\n\
	;#codeblock,8,4,\n\
	loop:\n\
	cmp ebx, ecx\n\
	je end\n\
	;#codeblock,13,1,\n\
	movzx edx, byte [ebc]\n\
	add eax, edx\n\
	inc ebx\n\
	jmp loop\n\
	;#codeblock,8,15,\n\
	end:\n\
	ret\
	";

	let _text2 = "\
	;#codeblock,0,10,\n\
	;add array elements [bytes]\n\
	;start of array in ebx;length in ecx;sum in eax \n\
	myfunc:\n\
	add ecx, ebx\n\
	xor eax, eax\n\
	;#codeblock,8,4,\n\
	loop:\n\
	cmp ebx, ecx\n\
	je end\n\
	movzx edx, byte [ebc]\n\
	add eax, edx\n\
	inc ebx\n\
	jmp loop\n\
	;#codeblock,8,15,\n\
	end:\n\
	ret\
	";

	let text3 = "\
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
	";*/

	let text3 = std::fs::read_to_string("asm.txt").unwrap();

	let mut data = parser::parse(&text3);
	splitter::split(&mut data);
	linker::link(&data);

	print_code(&data);

	let data = MyData{ code: CodeBlocks::new(data), mouse_click_pos: None, mouse_pos: Point::new(0.0,0.0), drag_mode: false,/* s1: "Hello world 1".to_string(), s2: "Hello world 2".to_string(), count: 0*/};
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window)/*.log_to_console()*/.launch(data).expect("launch failed");
}

fn ui_builder() -> impl Widget<MyData> {
    //let label = Label::new(|data: &u32, _env: &_| (*data).to_string()).padding(5.0).center();
    //let button = Button::new("increment").on_click(|_ctx, data, _env| *data += 1).padding(5.0);
    //Flex::column().with_child(label).with_child(button)
	let button = Button::new("print").on_click(|_ctx, data: &mut MyData, _env| print_code(&data.code.borrow())).padding(5.0);

	//let button2 = Button::new("add to vec").on_click(|_ctx, data: &mut MyData, _env| {data.code.borrow_mut().pop();}).padding(5.0);

	//&Rc<Vec<Rc<RefCell<CodeBlock>>>>
	/////let map = lens::Map::new(|vec: &MyData| vec.code.borrow()[0].borrow().text.clone(), |vec: &mut MyData, data| vec.code.borrow_mut()[0].borrow_mut().text = data);
	/////let mytext_box = TextBox::multiline().fix_size(500.0,500.0).lens(map);

	//mytext_box..fix_width(400.0).fix_height(400.0);

	let codeblockwindow = CodeBlockWindow::new();

	//SizedBox::new(druid::widget::Label::new("hello!")

	//SizedBox::new(codeblockwindow).width(2000.0).height(2000.0)

	let mut clip_box = clip_box::ClipBox::new(codeblockwindow);

	clip_box.set_do_clamping(false);

	let interceptor = Interceptor::new(clip_box).set_event_handler(&|child,ctx,event,data: &mut MyData,env| {
		match event {
			Event::MouseUp(_) => data.mouse_click_pos=None,
			Event::MouseDown(e) => {
				ctx.request_focus();
				ctx.set_active(true);

				if e.mods.ctrl() {
					data.mouse_click_pos=Some(e.pos);
					return;
				}
			},
			Event::MouseMove(e) => {
				data.mouse_pos = e.pos;
				if e.mods.ctrl() {
					if let Some(pos) = data.mouse_click_pos {
						child.pan_by(pos-e.pos);	// TODO: doesnt allow negative offsets
						//child.with_port(|f| f.rect=(f.rect.with_origin(f.rect.origin()+(pos-e.pos))));		//doesnt allow negative offsets too
						data.mouse_click_pos=Some(e.pos);
						ctx.request_update();
						return;
					}
				}
			},
			Event::KeyDown(e) if e.code==Code::KeyD && e.mods.ctrl() => data.drag_mode=true,
			Event::KeyDown(e) if e.code==Code::Escape => data.drag_mode=false,
			Event::KeyDown(e) if e.code==Code::Insert && e.mods.ctrl() => {
				data.code.borrow_mut().push(Rc::new(RefCell::new(CodeBlock{
					pos: data.mouse_pos,
					size: Default::default(),
					text: "".to_string(),					next: Default::default(),
					next_branch: Default::default()
				})))},
			_ => ()
		}
		child.event(ctx,event,data,env)
	});

	//let map2 = lens::Map::new(|vec: &MyData| vec.code.borrow()[1].borrow().text.clone(), |vec: &mut MyData, data| vec.code.borrow_mut()[1].borrow_mut().text = data);
	//let textbox2 = TextBox::new().lens(map2);

	///////let textbox3 = TextBox::new().lens(MyData::s1);

	//inter

	//Flex::column().with_child(button).with_child(textbox3).with_child(inter/*.fix_size(100.0,100.0)*/)
//.fix_size(1000.0,1000.0)
	Flex::column().with_child(button).with_flex_child(Padding::new(10.0,interceptor),1.0).debug_paint_layout()

	//let mut mywid = MyWidget::new();
    //mywid.add(mytext_box.lens(MyData::code));
	//mywid.add(TextBox::new().lens(MyData::s2));
	//mywid.add(Label::new("Hi!"));

	//Flex::column().with_child(button).with_child(TextBox::new().lens(MyData::s2)).with_child(TextBox::new().lens(MyData::s1)).with_flex_child(Flex::row().with_flex_child(mywid,1.0).with_child(Label::new("Hello!")),1.0)
}

/*

struct WidgetHolder<T> where T: Data {
	wid: WidgetPod<T, Box<dyn Widget<T>>>,
	//pos: Vec2,
}

struct MyWidget<T> where T: Data {
	widvec: Vec<WidgetHolder<T>>,
	dragged: Option<usize>,
    ctrlpressed: bool,
    ctrlDpressed: bool,
    mouseclickpos: Option<Point>,
    offset: Vec2,
}

impl<T> Widget<T> for MyWidget<T> where T: Data {
	fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env
    ){
		//println!("event {:?}",event);

		match event {
			Event::MouseDown(e) => {
				ctx.request_focus();
				ctx.set_active(true);
				let mut count: usize = 0;
                if !e.mods.ctrl() {
                    if self.ctrlDpressed {
                        for w in self.widvec.iter_mut() {
                            //let pos=(w.pos+self.offset).to_point();
                            if w.wid.layout_rect().contains(e.pos) {
                                //println!("dragged");
                                self.mouseclickpos=Some(e.pos-w.pos);
								data.
                                self.dragged = Some(count);
                                break;
                            }
                            count += 1;
                        }
                    }
                }else{
                    //println!("mouse down");
                    self.mouseclickpos=Some(e.pos-self.offset);
                }
			},
			Event::MouseUp(_) => {
                //println!("mouse up");
				ctx.set_active(false);
				self.dragged=None;
                self.mouseclickpos=None;
			},
			Event::MouseMove(e) => {
				if ctx.is_hot() && !ctx.has_focus() {
					ctx.request_focus();
				}
				if let Some(i) = self.dragged {
                    if let Some(pos) = self.mouseclickpos {
                        //println!("mouse move");
                        self.widvec[i].pos = e.pos.to_vec2()-pos.to_vec2();
                        ctx.request_layout();
                    }
				}

                if e.mods.ctrl() {
                    if let Some(pos) = self.mouseclickpos {
                        self.offset=e.pos-pos;
                        ctx.request_layout();
                    }
                }
			},
			Event::KeyDown(e) => {
                //println!("event KeyDown: {:?}", e);
                match e.code {
					Code::KeyD if e.mods.ctrl() => { self.ctrlDpressed = true; },
					Code::ControlLeft => { self.ctrlpressed = true; },
					Code::Escape => { self.ctrlDpressed = false; },
					_ => {},
				}
            },
			Event::KeyUp(e) => {
                //println!("event KeyUp: {:?}", e);
                match e.code {
                    Code::ControlLeft => {self.ctrlpressed=false;},
                    _ => {},
                }
            },
		}
		for w in self.widvec.iter_mut() {
			w.wid.event(ctx, event, data, env);
		}
	}

	fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env
    ){
		//println!("lifecycle {:?}", event);
		for w in self.widvec.iter_mut() {
			w.wid.lifecycle(ctx, event, data, env);
		}
	}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env){
		//println!("update");
		ctx.request_paint();
	}

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env
    ) -> Size {
		let childbc = BoxConstraints::new(Size::ZERO, Size::new(std::f64::INFINITY, std::f64::INFINITY));
		for w in self.widvec.iter_mut() {
			let size = w.wid.layout(ctx, &childbc, data, env);
			let rect=Rect::from_origin_size(w.pos.to_point()+self.offset, size);
			w.wid.set_layout_rect(ctx, data, env, rect);
		}
		bc.max()
	}

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env){
		let clip_rect = ctx.size().to_rect();
		ctx.clip(clip_rect);
		ctx.stroke(clip_rect,&env.get(theme::PRIMARY_LIGHT),5.0);
		let brush= ctx.solid_brush(Color::rgb8(255,0,0));

		let size=20.0;
		let color = Color::rgb8(255,255,255);
		let layout = ctx.text().new_text_layout("my text\nmy new line").text_color(color).build().unwrap();
        ctx.draw_text(&layout,(0.0,size));

		for w in self.widvec.iter_mut() {
			ctx.stroke(w.wid.layout_rect(),&env.get(theme::PRIMARY_LIGHT),2.0);
			w.wid.paint(ctx, data, env);
			ctx.stroke(w.wid.layout_rect().with_size(Size::new(5.0,5.0)) +Vec2::new(-5.0,-5.0), &brush,5.0);
		}
	}
}

impl<T> MyWidget<T>  where T: Data{
	fn new() -> MyWidget<T> {
		MyWidget{widvec: Vec::new(), dragged: None, ctrlpressed: false, ctrlDpressed: false, mouseclickpos: None, offset: Vec2::new(0.0,0.0)}
	}

	fn add(&mut self, w: impl Widget<T> + 'static) {
		self.widvec.push(WidgetHolder{wid: WidgetPod::new(Box::new(w))});
	}
}*/