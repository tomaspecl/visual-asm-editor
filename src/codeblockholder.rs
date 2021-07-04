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
use crate::MyData;

use std::rc::{Weak, Rc};
use std::cell::RefCell;
use druid::{Widget, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, LayoutCtx, Event, Env, UpdateCtx, WidgetPod, Code};
use druid::kurbo::{Size, Point};

//holds a Weak reference to a CodeBlock
//has fn has_valid_reference() -> bool      returns false when Weak reference is invalid
//contains Widget<CodeBlock>
//has event handler for mouse, when Ctrl+D is pressed then activated drag mode, mouse drags cause the CodeBlock.pos to change accordingly
//has fn get_pos() -> Vec2      returns CodeBlock.pos

pub struct CodeBlockHolder {
    pub block: Weak<RefCell<CodeBlock>>,
    pub child: WidgetPod<CodeBlock, Box<dyn Widget<CodeBlock>>>
}

impl CodeBlockHolder {
    pub fn has_valid_reference(&self) -> bool { self.block.strong_count()>0 }
    pub fn get_pos(&self) -> Point { self.get_codeblock().borrow().pos }
    pub fn get_codeblock(&self) -> Rc<RefCell<CodeBlock>> { self.block.upgrade().unwrap() }
    pub fn new(block: Weak<RefCell<CodeBlock>>, child: impl Widget<CodeBlock> + 'static) -> Self {
        CodeBlockHolder{
            block,
            child: WidgetPod::new(Box::new(child)),
        }
    }
}

impl Widget<MyData> for CodeBlockHolder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MyData, env: &Env) {
        match event {
            Event::MouseDown(e) if data.drag_mode && self.child.layout_rect().contains(e.pos) => {
                data.mouse_click_pos=Some(e.pos)
            },
            Event::MouseUp(_) => {
                data.mouse_click_pos=None
            },
            Event::MouseMove(e) if self.child.layout_rect().contains(e.pos) => {
                if let Some(pos)=data.mouse_click_pos {
                    self.block.upgrade().unwrap().borrow_mut().pos+=e.pos-pos;
                    data.mouse_click_pos=Some(e.pos);
                }
            },
            Event::KeyDown(k) if k.code==Code::Delete && k.mods.ctrl() && self.child.is_hot()=> {
                //delete itself
                println!("deleting {:?}", self.block.as_ptr());
                let block = self.block.upgrade().unwrap();
                data.code.borrow_mut().retain(|x| !Rc::ptr_eq(x, &block));
                ctx.set_handled();
                ctx.resign_focus();
                ctx.set_active(false);
                return;
            },
            _ => ()
        }

        let codeblock = self.get_codeblock();
        let child_data = &mut*codeblock.borrow_mut();

        let text = child_data.text.clone();

        self.child.event(ctx,event,child_data,env);

        if !text.eq(&child_data.text) {
            data.code.text_changed = true;
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &MyData, env: &Env) {
        self.child.lifecycle(ctx,event,&*self.get_codeblock().borrow(),env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &MyData, _data: &MyData, env: &Env) {
        self.child.update(ctx,&*self.get_codeblock().borrow(),env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &MyData, env: &Env) -> Size {
        let size = self.child.layout(ctx,bc,&*self.get_codeblock().borrow(),env);
        self.get_codeblock().borrow_mut().size=size;
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &MyData, env: &Env) {
        self.child.paint_always(ctx,&*self.get_codeblock().borrow(),env);
    }
}
