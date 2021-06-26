use crate::codeblock::CodeBlock;

use kurbo::Line;
use druid::widget::TextBox;
use druid::*;

//will contain TextBox and will draw controll flow arrows
//will take CodeBlock and lens it to text for its TextBox


//code block holder will contain some dyn Widget (this textboxholder)
//and it will draw its widget and respond to mouse and keyboard, do dragging etc... and contain some other controll of the codeblock

pub struct TextBoxHolder {
    pub child: TextBox<String>
}

const TEXT_SIZE: f64 = 15.0;

impl Widget<CodeBlock> for TextBoxHolder { 
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CodeBlock, env: &Env) {
        self.child.event(ctx, event, &mut data.text, env);
    }
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &CodeBlock, env: &Env) {
        self.child.lifecycle(ctx, event, &data.text, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CodeBlock, data: &CodeBlock, env: &Env) {
        self.child.update(ctx, &old_data.text, &data.text, env);
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &CodeBlock, env: &Env) -> Size {
        self.child.set_text_size(TEXT_SIZE);
        self.child.layout(ctx, bc, &data.text, env)
    }
    fn paint(&mut self, ctx: &mut PaintCtx, data: &CodeBlock, env: &Env) {
        self.child.paint(ctx, &data.text, env);

        let size = data.size;

        if let Some(next) = data.next.upgrade() {

            let size_next = next.borrow().size;

            let p0 = Point::new(size.width/2.0,size.height) /*+ data.pos.to_vec2()*/;
            let p1 = Point::new(size_next.width/2.0,0.0) + next.borrow().pos.to_vec2() - data.pos.to_vec2();
            let shape = Line::new(p0, p1);
            let brush = Color::rgb8(128, 0, 0);
            ctx.stroke(shape,&brush,5.0);   // TODO: better arrows
        }
        if let Some(next) = data.next_branch.upgrade() {

            let line = data.next_branch_line;

            let size_next = next.borrow().size;

            //self.child.layout_rect()
            ctx.with_save(|ctx|{
                //ctx.transform(Affine::translate(self.child.layout_rect().origin().to_vec2()));
                //let p0 = Point::new(size.width/2.0,size.height) + self.get_pos().to_vec2();
                let p0 = Point::new(size.width/2.0,10.0 + TEXT_SIZE*line as f64) /*+ data.pos.to_vec2()*/;    // TODO: better offset and use text size
                let p1 = Point::new(size_next.width/2.0,0.0) + next.borrow().pos.to_vec2() - data.pos.to_vec2();
                let shape = Line::new(p0, p1);
                let brush = Color::rgb8(128, 0, 0);
                ctx.stroke(shape,&brush,5.0);   // TODO: better arrows, from correct line
            });
        }
    }
}