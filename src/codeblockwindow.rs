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
use crate::codeblockholder_nowidgetpod::CodeBlockHolder;
use crate::codeblock::CodeBlock;
use crate::textboxholder::TextBoxHolder;

use druid::{Rect, RenderContext, Affine, Widget, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, LayoutCtx, Event, Env, UpdateCtx, WidgetExt};
use druid::kurbo::Size;
use std::rc::Rc;
use druid::widget::{TextBox, SizedBox};

//renders CodeBlocks
//has internal Vec of CodeBlockHolder
//checks each CodeBlockHolder.has_valid_reference(), if false then removes the CodeBlockHolder from the Vec

pub struct CodeBlockWindow {
    children: Vec<CodeBlockHolder>
}

impl CodeBlockWindow {
    pub fn new() -> Self {
        CodeBlockWindow{
            children: Vec::new(),
        }
    }
}

impl Widget<MyData> for CodeBlockWindow {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MyData, env: &Env) {
        for w in &mut self.children {
            w.event(ctx,&event,data,env);
            if ctx.is_handled() {break;}
        }
        crate::splitter::split(&mut*data.code.borrow_mut());
        crate::linker::link(&mut*data.code.borrow_mut());
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &MyData, env: &Env) {
        let children = &mut self.children;
        let blocks = data.code.borrow();

        children.retain(|e| e.has_valid_reference());

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
                if !contained {//.fix_size(200.0,200.0)
                    children.push(CodeBlockHolder::new(Rc::downgrade(&block), SizedBox::new(TextBoxHolder{child: TextBox::multiline()}).width(200.0)/*.padding(0.0)*/));
                }
            }
        }

        for w in &mut self.children {
            w.lifecycle(ctx,event,data,env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &MyData, data: &MyData, env: &Env) {
        self.children.retain(|e| e.has_valid_reference());

        for w in self.children.iter_mut() {
            w.update(ctx,old_data,data,env);
        }
        ctx.request_layout();
        //ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &MyData, env: &Env) -> Size {
        let childbc = BoxConstraints::new(Size::ZERO, Size::new(f64::INFINITY, f64::INFINITY));

        for w in &mut self.children {
            w.layout(ctx,&childbc,data,env);
            //let pos = w.get_pos();
            //w.child.set_origin(ctx,&w.get_codeblock().borrow(),env,pos);
        }
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &MyData, env: &Env) {
        /*for w in &mut self.children {
            let pos = w.get_pos();
            ctx.with_save(|ctx|{
                ctx.transform(Affine::translate(pos.to_vec2()));
                w.paint(ctx,data,env);
            });
        }*/
        for w in &mut self.children {
            w.paint(ctx,data,env);
        }
    }
}