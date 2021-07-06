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

use druid::text::{ImeHandlerRef, Movement, TextAction};
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
        match event {
            Event::Command(c) => {
                if let Some(()) = c.get(druid::Selector::new("move_cursor_to_end")) {
                    let input_handler = self.child.text_mut().input_handler();
                    if let Some(mut x) = input_handler.acquire(true) {
                        x.handle_action(TextAction::Move(Movement::ParagraphEnd));
                        //ctx.invalidate_text_input(text::ImeInvalidation::SelectionChanged);
                    }
                    input_handler.release();
                    self.child.event(ctx, &Event::ImeStateChange, &mut data.text, env);
                }
            },
            _ => ()
        }
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

        let start = data;

        if let Some(next) = data.next.upgrade() {
            let end = &*next.borrow();

            let arrow_start = Point::new(start.size.width*0.5, start.size.height);
            let arrow_end = end.pos + Vec2::new(end.size.width*0.5, 0.0) - start.pos.to_vec2();

            paint_arrow_from_bottom(ctx, arrow_start, arrow_end);
        }
        if let Some(next) = data.next_branch.upgrade() {
            let end = &*next.borrow();

            let side = preffered_side(start, end);

            let text_layout = &self.child.text().borrow().layout;
            let line_offset = text_layout.point_for_text_position(data.next_branch_line_offset);

            let arrow_start = match &side {
                Side::Left => line_offset,
                Side::Right => line_offset + Vec2::new(start.size.width, 0.0),
            };
            let arrow_end = end.pos + Vec2::new(end.size.width*0.5, 0.0) - start.pos.to_vec2();

            paint_arrow_from_side(ctx, arrow_start, side, arrow_end);
        }
    }
}

fn paint_arrow_from_bottom(ctx: &mut PaintCtx, start: Point, end: Point) {
    let p1 = start + Vec2::new(0.0,10.0);
    let p4 = end - Vec2::new(0.0,10.0);
    let center = p1.midpoint(p4);   // TODO: pick center based on preffered side
    let (p2,p3) = if p1.y < p4.y {
        (Point::new(p1.x,center.y),Point::new(p4.x,center.y))
    }else{
        (Point::new(center.x,p1.y),Point::new(center.x,p4.y))
    };

    let brush = Color::rgb8(128, 0, 0);
    
    let line1 = Line::new(start, p1);
    let line2 = Line::new(p1, p2);
    let line3 = Line::new(p2, p3);
    let line4 = Line::new(p3, p4);
    let line5 = Line::new(p4, end);
    
    ctx.stroke(line1,&brush,5.0);   // TODO: make it actual arrows instead of just lines
    ctx.stroke(line2,&brush,5.0);
    ctx.stroke(line3,&brush,5.0);
    ctx.stroke(line4,&brush,5.0);
    ctx.stroke(line5,&brush,5.0);
}

fn paint_arrow_from_side(ctx: &mut PaintCtx, start: Point, preffered_side: Side, end: Point) {
    let p3 = end - Vec2::new(0.0,10.0);
    let (p1,p2) = match preffered_side {
        Side::Left => {
            let p1 = start - Vec2::new(10.0,0.0);
            let p2 = if p1.x > p3.x && p1.y < p3.y {
                Point::new(p3.x,p1.y)
            }else{
                Point::new(p1.x,p3.y)
            };
            (p1,p2)
        },
        Side::Right => {
            let p1 = start + Vec2::new(10.0,0.0);
            let p2 = if p1.x < p3.x && p1.y < p3.y {
                Point::new(p3.x,p1.y)
            }else{
                Point::new(p1.x,p3.y)
            };
            (p1,p2)
        }
    };

    let brush = Color::rgb8(128, 0, 0);

    let line1 = Line::new(start, p1);
    let line2 = Line::new(p1, p2);
    let line3 = Line::new(p2, p3);
    let line4 = Line::new(p3, end);

    ctx.stroke(line1,&brush,5.0);   // TODO: make it actual arrows instead of just lines
    ctx.stroke(line2,&brush,5.0);
    ctx.stroke(line3,&brush,5.0);
    ctx.stroke(line4,&brush,5.0);
}

enum Side {
    Left,
    Right
}

// TODO: better preffering
fn preffered_side(start: &CodeBlock, end: &CodeBlock) -> Side {
    let start_center = start.pos+start.size.to_vec2()*0.5;
    let end_center = end.pos+end.size.to_vec2()*0.5;

    if start_center.x < end_center.x {
        Side::Right
    }else{
        Side::Left
    }
}
