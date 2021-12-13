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

#[allow(unused_variables)]
pub trait LimelightController: 'static {
    fn draw(&mut self, renderer: &mut Renderer, ts: f64) -> Result<ShouldRequestAnimationFrame>;

    fn handle_key_down(&mut self, key: KeyCode) -> ShouldRequestAnimationFrame {
        false
    }

    fn handle_key_up(&mut self, key: KeyCode) -> ShouldRequestAnimationFrame {
        false
    }

    fn handle_drag(&mut self, x: u32, y: u32) -> ShouldRequestAnimationFrame {
        false
    }

    fn handle_scroll(&mut self, x: u32, y: u32) -> ShouldRequestAnimationFrame {
        false
    }

    fn handle_zoom(&mut self, amount: u32) -> ShouldRequestAnimationFrame {
        false
    }
}

pub struct LimelightComponent<Controller: LimelightController> {
    canvas_ref: NodeRef,
    renderer: Option<Renderer>,
    render_handle: Option<AnimationFrame>,
    keydown_handler: Option<EventListener>,
    keyup_handler: Option<EventListener>,
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
pub struct ControllerProps<Controller: LimelightController> {
    controller: Rc<RefCell<Controller>>,
    height: u32,
    width: u32,
}

impl<Controller: LimelightController> Default for ControllerProps<Controller>
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

impl<Controller: LimelightController> PartialEq for ControllerProps<Controller> {
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

    type Properties = ControllerProps<Controller>;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            renderer: None,
            render_handle: None,
            keydown_handler: None,
            keyup_handler: None,
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
                let should_render = (*ctx.props().controller)
                    .borrow_mut()
                    .handle_key_down(event.key().as_str().into());
                if should_render {
                    self.request_render(ctx);
                }
            }
            Msg::KeyUp(event) => {
                let should_render = (*ctx.props().controller)
                    .borrow_mut()
                    .handle_key_up(event.key().as_str().into());
                if should_render {
                    self.request_render(ctx);
                }
            }
            e => log::info!("Unhandled event {:?}", e),
        }

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        let link = ctx.link();

        html! {
            <canvas
                height={props.height.to_string()}
                width={props.width.to_string()}
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
                    move |event| {
                        event.prevent_default();
                        callback.emit(event.clone().dyn_into().unwrap())
                    },
                ));
            }
            {
                let callback = ctx.link().callback(Msg::KeyUp);
                self.keyup_handler = Some(EventListener::new_with_options(
                    &window().unwrap(),
                    "keyup",
                    options,
                    move |event| {
                        event.prevent_default();
                        callback.emit(event.clone().dyn_into().unwrap())
                    },
                ));
            }

            self.renderer = Some(Renderer::new(gl));

            self.request_render(ctx);
        }
    }
}
