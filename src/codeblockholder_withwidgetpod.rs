use crate::codeblock::CodeBlock;
use crate::MyData;

use std::rc::{Weak, Rc};
use std::cell::RefCell;
use druid::{Widget, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, LayoutCtx, Event, Env, UpdateCtx, WidgetPod, Color, Code};
use druid::kurbo::{Size, Line, Point, Affine};
use druid::widget::prelude::RenderContext;

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
    /*pub fn new(block: Weak<RefCell<CodeBlock>>, child: druid::widget::TextBox<CodeBlock>) -> Self {
        CodeBlockHolder{
            block,
            child: WidgetPod::new(child),
        }
    }*/
}

impl Widget<MyData> for CodeBlockHolder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut MyData, env: &Env) {
        //if !ctx.is_hot() {
        //    ctx.set_active(false);
        //}
        //dbg!(event);
        match event {
            Event::MouseDown(e) if data.drag_mode && self.child.layout_rect().contains(e.pos) => {
                //ctx.request_focus();
                //ctx.set_active(true);
                data.mouse_click_pos=Some(e.pos)
            },
            Event::MouseUp(_) => {
                data.mouse_click_pos=None
            },
            Event::MouseMove(e) if self.child.layout_rect().contains(e.pos) => {
                if let Some(pos)=data.mouse_click_pos {
                    //println!("holder event {} {} {}",self.block.upgrade().unwrap().borrow().pos, self.child.layout_rect(), e.pos);
                    self.block.upgrade().unwrap().borrow_mut().pos+=e.pos-pos;
                    data.mouse_click_pos=Some(e.pos);
                }
                //ctx.request_layout();
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
        self.child.event(ctx,event,&mut*self.get_codeblock().borrow_mut(),env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &MyData, env: &Env) {
        self.child.lifecycle(ctx,event,&*self.get_codeblock().borrow(),env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &MyData, _data: &MyData, env: &Env) {
        self.child.update(ctx,&*self.get_codeblock().borrow(),env);
        //ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &MyData, env: &Env) -> Size {
        let size = self.child.layout(ctx,bc,&*self.get_codeblock().borrow(),env);
        self.get_codeblock().borrow_mut().size=size;
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &MyData, env: &Env) {
        self.child.paint(ctx,&*self.get_codeblock().borrow(),env);
    }
}