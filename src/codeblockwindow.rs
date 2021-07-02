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
use crate::MyData;
use crate::codeblockholder::CodeBlockHolder;
use crate::codeblock::CodeBlock;
use crate::textboxholder::TextBoxHolder;

use druid::{Code, RenderContext, Widget, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, LayoutCtx, Event, Env, UpdateCtx};
use druid::kurbo::{Size, Point, Vec2};
use std::cell::RefCell;
use std::rc::Rc;
use druid::widget::{TextBox, SizedBox};

//renders CodeBlocks
//has internal Vec of CodeBlockHolder
//checks each CodeBlockHolder.has_valid_reference(), if false then removes the CodeBlockHolder from the Vec

pub struct CodeBlockWindow {
    offset: Point,
    children: Vec<CodeBlockHolder>
}

#[allow(dead_code)]
impl CodeBlockWindow {
    pub fn new() -> Self {
        CodeBlockWindow{
            offset: (0.0,0.0).into(),
            children: Vec::new(),
        }
    }

    pub fn pan_to(&mut self, origin: Point) {
        self.offset = origin;
    }

    pub fn pan_by(&mut self, delta: Vec2) {
        self.offset += delta;
    }

    /// Returns true if children changed
    fn manage_children(&mut self, data: &MyData) -> bool {
        let children = &mut self.children;
        let blocks = data.code.borrow();

        let mut children_changed = false;

        children.retain(|e| e.has_valid_reference());   // TODO: return true when delete happened

        if children.len() != blocks.len() {
            //add new children
            for block in &*blocks {
                let mut contained = false;
                for child in &*children {
                    if Rc::ptr_eq(block, &child.block.upgrade().unwrap()) {
                        contained = true;
                        break;
                    }
                }
                if !contained {
                    children.push(CodeBlockHolder::new(Rc::downgrade(&block), SizedBox::new(TextBoxHolder{child: TextBox::multiline()}).width(200.0)/*.padding(0.0)*/));
                    children_changed = true;
                }
            }
        }

        children_changed
    }
}

impl Widget<MyData> for CodeBlockWindow {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MyData, env: &Env) {
        // TODO: clip (mouse) events by view_rectangle
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
						self.pan_by(e.pos-pos);
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
					text: "".to_string(),
					next: Default::default(),
					next_branch: Default::default(),
					next_branch_line: 0,
				})));
                if self.manage_children(data) {
                    ctx.children_changed();
                }
                return;
			},
			_ => ()
		}

        for w in &mut self.children {
            w.event(ctx,event,data,env);
            if ctx.is_handled() {break;}
        }
        crate::splitter::split(&mut*data.code.borrow_mut());
        crate::linker::link(&mut*data.code.borrow_mut());

        if self.manage_children(data) {
            ctx.children_changed();
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &MyData, env: &Env) {
        if self.manage_children(data) {
            ctx.children_changed();
        }

        for w in &mut self.children {
            w.lifecycle(ctx,event,data,env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &MyData, data: &MyData, env: &Env) {
        if self.manage_children(data) {
            ctx.children_changed();
        }

        ctx.request_layout();

        for w in self.children.iter_mut() {
            w.update(ctx,old_data,data,env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &MyData, env: &Env) -> Size {
        let childbc = BoxConstraints::new(Size::ZERO, Size::new(f64::INFINITY, f64::INFINITY));

        for w in &mut self.children {
            let _size = w.layout(ctx,&childbc,data,env);
            let pos = w.get_pos();
            w.child.set_origin(ctx,&w.get_codeblock().borrow(),env,pos+self.offset.to_vec2());
        }
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &MyData, env: &Env) {
        let view_rectangle = ctx.size().to_rect();
        ctx.clip(view_rectangle);
        for w in &mut self.children {
            w.paint(ctx,data,env);
        }
    }
}