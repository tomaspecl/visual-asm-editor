use druid::{Widget, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, LayoutCtx, Event, Env, UpdateCtx};
use druid::kurbo::Size;

pub struct Interceptor<'a, T, W: Widget<T>> {
    pub child: W,
    event: Option<&'a dyn Fn(&mut W, &mut EventCtx, &Event, &mut T, &Env)>,
    lifecycle: Option<&'a dyn Fn(&mut W, &mut LifeCycleCtx, &LifeCycle, &T, &Env)>,
    update: Option<&'a dyn Fn(&mut W, &mut UpdateCtx, &T, &T, &Env)>,
    layout: Option<&'a dyn Fn(&mut W, &mut LayoutCtx, &BoxConstraints, &T, &Env)->Size>,
    paint: Option<&'a dyn Fn(&mut W, &mut PaintCtx, &T, &Env)>,
}

#[allow(dead_code)]
impl<'a, T, W: Widget<T>> Interceptor<'a, T, W> {
    pub fn new(child: W) -> Self {
        Self{
            child,
            event: None,
            lifecycle: None,
            update: None,
            layout: None,
            paint: None,
        }
    }

    pub fn set_event_handler(mut self, f: &'a dyn Fn(&mut W, &mut EventCtx, &Event, &mut T, &Env)) -> Self {
        self.event=Some(f);
        self
    }

    pub fn set_lifecycle_handler(mut self, f: &'a dyn Fn(&mut W, &mut LifeCycleCtx, &LifeCycle, &T, &Env)) -> Self {
        self.lifecycle=Some(f);
        self
    }

    pub fn set_update_handler(mut self, f: &'a dyn Fn(&mut W, &mut UpdateCtx, &T, &T, &Env)) -> Self {
        self.update=Some(f);
        self
    }

    pub fn set_layout_handler(mut self, f: &'a dyn Fn(&mut W, &mut LayoutCtx, &BoxConstraints, &T, &Env)->Size) -> Self {
        self.layout=Some(f);
        self
    }

    pub fn set_paint_handler(mut self, f: &'a dyn Fn(&mut W, &mut PaintCtx, &T, &Env)) -> Self {
        self.paint=Some(f);
        self
    }
}

impl<T, W: Widget<T>> Widget<T> for Interceptor<'_, T, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Some(f) = self.event {
            f(&mut self.child, ctx, event, data, env);
        }else{
            self.child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let Some(f) = self.lifecycle {
            f(&mut self.child, ctx, event, data, env);
        }else{
            self.child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if let Some(f) = self.update {
            f(&mut self.child, ctx, old_data, data, env);
        }else{
            self.child.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        if let Some(f) = self.layout {
            f(&mut self.child, ctx, bc, data, env)
        }else{
            self.child.layout(ctx, bc, data, env)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if let Some(f) = self.paint {
            f(&mut self.child, ctx, data, env);
        }else{
            self.child.paint(ctx, data, env);
        }
    }
}