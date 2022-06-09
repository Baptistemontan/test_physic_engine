use std::{f64::consts::TAU, time::Duration};

// use wasm_timer::Instant;

use gloo::timers::callback::Interval;
use physic_engine::matrix::static_vector::StaticColumnVector;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use crate::circle::Circle;
mod circle;

const TICK_INTERVAL: u32 = 1000 / 60;

const SIZE: usize = 400;
const RADIUS: f64 = (SIZE as f64) / 2.1;

enum Msg {
    Tick,
    Click(i32, i32),
    Reset,
}

struct App {
    // last_render: Instant,
    middle: StaticColumnVector<2>,
    _interval: Interval,
    circles: Vec<Circle>,
    gravity: StaticColumnVector<2>,
}

const CANVAS_ID: &str = "simulation-canvas";

fn get_canvas(id: &str) -> Option<HtmlCanvasElement> {
    let document = web_sys::window()?.document()?;
    let canvas = document.get_element_by_id(id)?;
    let canvas = canvas.dyn_into().ok()?;
    Some(canvas)
}

fn get_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

impl App {
    fn update(&mut self) {
        // let now = Instant::now();
        // let dt = now - self.last_render;
        // self.last_render = now;
        let dt = Duration::from_millis(TICK_INTERVAL as u64);
        Circle::update_all(&mut self.circles, &self.middle, RADIUS, dt);
    }

    fn render(&self) {
        let canvas = get_canvas(CANVAS_ID).expect("canvas not found");
        let ctx = get_context(&canvas).expect("context could'nt be created");

        ctx.clear_rect(0.0, 0.0, SIZE as f64, SIZE as f64);

        ctx.begin_path();

        let [cx, cy] = self.middle.map(|x| x);

        ctx.move_to(cx + RADIUS, cy);

        ctx.arc(cx, cy, RADIUS, 0.0, TAU).unwrap();

        for circle in self.circles.iter() {
            circle.render(&ctx);
        }

        ctx.stroke();
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let interval = {
            let link = ctx.link().clone();
            Interval::new(TICK_INTERVAL, move || link.send_message(Msg::Tick))
        };
        let gravity = SIZE as f64 / 3.0;

        let gravity = [0.0, gravity].into();

        // let last_render = Instant::now();

        let mid = SIZE as f64 / 2.0;
        let middle = StaticColumnVector::from([mid, mid]);

        Self {
            // last_render,
            middle,
            _interval: interval,
            gravity,
            circles: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.update();
                self.render();
                false
            }
            Msg::Click(x, y) => {
                let position = [x as f64, y as f64].into();
                self.circles
                    .push(Circle::new(RADIUS * 0.05, position, self.gravity.clone()));
                false
            }
            Msg::Reset => {
                self.circles = vec![];
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|e: MouseEvent| {
            let x = e.client_x();
            let y = e.client_y();
            Msg::Click(x, y)
        });

        let on_reset = ctx.link().callback(|_| Msg::Reset);

        let size = SIZE.to_string();

        html! {
            <div>
                <canvas id={CANVAS_ID} {onclick} width={size} height={size.clone()} />
                <button onclick={on_reset}>{"Reset"}</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
