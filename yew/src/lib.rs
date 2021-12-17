mod key_event;

use anyhow::Result;
use gloo_events::{EventListener, EventListenerOptions};
use gloo_render::{request_animation_frame, AnimationFrame};
pub use key_event::KeyCode;
use limelight::renderer::Renderer;
use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext};
use yew::{html, Component, KeyboardEvent, MouseEvent, NodeRef, Properties, WheelEvent};

pub type ShouldRequestAnimationFrame = bool;
pub type ShouldCancelEvent = bool;

#[allow(unused_variables)]
pub trait LimelightController: 'static {
    fn draw(&mut self, renderer: &mut Renderer, ts: f64) -> Result<ShouldRequestAnimationFrame>;

    fn handle_key_down(
        &mut self,
        key: KeyCode,
    ) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        (false, false)
    }

    fn handle_key_up(&mut self, key: KeyCode) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        (false, false)
    }

    fn handle_drag(&mut self, x: f32, y: f32) -> ShouldRequestAnimationFrame {
        false
    }

    fn handle_mousemove(&mut self, x: f32, y: f32) -> ShouldRequestAnimationFrame {
        false
    }

    fn handle_scroll(
        &mut self,
        x_amount: f32,
        y_amount: f32,
        x_position: f32,
        y_position: f32,
    ) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        (false, false)
    }

    fn handle_pinch(
        &mut self,
        amount: f32,
        x: f32,
        y: f32,
    ) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        (false, false)
    }
}

pub struct LimelightComponent<Controller: LimelightController> {
    canvas_ref: NodeRef,
    renderer: Option<Renderer>,
    render_handle: Option<AnimationFrame>,
    keydown_handler: Option<EventListener>,
    keyup_handler: Option<EventListener>,
    drag_origin: Option<(i32, i32)>,
    _ph: PhantomData<Controller>,
}

#[derive(Debug)]
pub enum Msg {
    Render(f64),
    MouseMove(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseWheel(WheelEvent),
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
}

#[derive(Properties)]
pub struct LimelightComponentProps<Controller: LimelightController> {
    pub controller: Rc<RefCell<Controller>>,
    pub height: i32,
    pub width: i32,
}

impl<Controller: LimelightController> Default for LimelightComponentProps<Controller>
where
    Controller: Default,
{
    fn default() -> Self {
        Self {
            controller: Rc::new(RefCell::new(Controller::default())),
            width: 600,
            height: 600,
        }
    }
}

impl<Controller: LimelightController> PartialEq for LimelightComponentProps<Controller> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.controller, &other.controller)
    }
}

impl<Controller: LimelightController> LimelightComponent<Controller> {
    fn request_render(&mut self, ctx: &yew::Context<Self>) {
        let render_callback = ctx.link().callback(Msg::Render);
        self.render_handle = Some(request_animation_frame(move |ts| render_callback.emit(ts)));
    }
}

impl<Controller: LimelightController> Component for LimelightComponent<Controller> {
    type Message = Msg;

    type Properties = LimelightComponentProps<Controller>;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            renderer: None,
            render_handle: None,
            keydown_handler: None,
            keyup_handler: None,
            drag_origin: None,
            _ph: PhantomData::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(ts) => {
                if let Some(renderer) = &mut self.renderer {
                    let should_render = (*ctx.props().controller)
                        .borrow_mut()
                        .draw(renderer, ts)
                        .unwrap();

                    if should_render {
                        self.request_render(ctx);
                    }
                }
            }
            Msg::KeyDown(event) => {
                let (should_render, should_cancel_event) = (*ctx.props().controller)
                    .borrow_mut()
                    .handle_key_down(event.key().as_str().into());
                if should_render {
                    self.request_render(ctx);
                }
                if should_cancel_event {
                    event.prevent_default();
                }
            }
            Msg::KeyUp(event) => {
                let (should_render, should_cancel_event) = (*ctx.props().controller)
                    .borrow_mut()
                    .handle_key_up(event.key().as_str().into());
                if should_render {
                    self.request_render(ctx);
                }
                if should_cancel_event {
                    event.prevent_default();
                }
            }
            Msg::MouseDown(e) => {
                self.drag_origin = Some((e.offset_x(), e.offset_y()));
            }
            Msg::MouseUp(_) => {
                self.drag_origin = None;
            }
            Msg::MouseMove(e) => {
                let (new_x, new_y) = (e.offset_x(), e.offset_y());

                if let Some((origin_x, origin_y)) = self.drag_origin {
                    let should_render = (*ctx.props().controller).borrow_mut().handle_drag(
                        2. * (new_x - origin_x) as f32 / ctx.props().width as f32,
                        2. * -(new_y - origin_y) as f32 / ctx.props().height as f32,
                    );

                    if should_render {
                        self.request_render(ctx);
                    }

                    self.drag_origin = Some((new_x, new_y));
                } else {
                    let should_render = (*ctx.props().controller).borrow_mut().handle_mousemove(
                        2. * new_x as f32 / ctx.props().width as f32 - 1.,
                        2. * -new_y as f32 / ctx.props().height as f32 + 1.,
                    );

                    if should_render {
                        self.request_render(ctx);
                    }
                }
            }
            Msg::MouseWheel(e) => {
                let scroll_amount_y = e.delta_y() as f32;
                let scroll_amount_x = e.delta_x() as f32;

                let pin_x = (2 * e.offset_x()) as f32 / ctx.props().width as f32 - 1.;
                let pin_y = -((2 * e.offset_y()) as f32 / ctx.props().height as f32 - 1.);

                let (should_render, should_cancel_event) = if e.ctrl_key() {
                    (*ctx.props().controller).borrow_mut().handle_pinch(
                        -scroll_amount_y,
                        pin_x,
                        pin_y,
                    )
                } else {
                    (*ctx.props().controller).borrow_mut().handle_scroll(
                        -scroll_amount_x as f32 * 2. / ctx.props().width as f32,
                        scroll_amount_y as f32 * 2. / ctx.props().height as f32,
                        pin_x,
                        pin_y,
                    )
                };

                if should_render {
                    self.request_render(ctx);
                }

                if should_cancel_event {
                    e.prevent_default();
                }
            }
        }

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        let link = ctx.link();
        let device_pixel_ratio = window().unwrap().device_pixel_ratio();

        html! {
            <canvas
                height={(props.height as f64 * device_pixel_ratio).to_string()}
                width={(props.width as f64 * device_pixel_ratio).to_string()}
                style={format!("width: {}px; height: {}px;", props.width, props.height)}
                onmousedown={link.callback(Msg::MouseDown)}
                onmousemove={link.callback(Msg::MouseMove)}
                onmouseup={link.callback(Msg::MouseUp)}
                onwheel={link.callback(Msg::MouseWheel)}
                onkeydown={link.callback(Msg::KeyDown)}
                ref={self.canvas_ref.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let gl: WebGl2RenderingContext = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            let options = EventListenerOptions::enable_prevent_default();
            {
                let callback = ctx.link().callback(Msg::KeyDown);
                self.keydown_handler = Some(EventListener::new_with_options(
                    &window().unwrap(),
                    "keydown",
                    options,
                    move |event| callback.emit(event.clone().dyn_into().unwrap()),
                ));
            }
            {
                let callback = ctx.link().callback(Msg::KeyUp);
                self.keyup_handler = Some(EventListener::new_with_options(
                    &window().unwrap(),
                    "keyup",
                    options,
                    move |event| callback.emit(event.clone().dyn_into().unwrap()),
                ));
            }

            self.renderer = Some(Renderer::new(gl));

            self.request_render(ctx);
        }
    }
}
