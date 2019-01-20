extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate wbg_rand;

use std::cell::RefCell;
use std::rc::Rc;
use wbg_rand::{Rng, wasm_rng};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[derive(Copy, Clone)]
struct Ball {
    x_speed: f64,
    y_speed: f64,
    radius: f64,
    x: f64,
    y: f64,
    x_acceleration: f64,
    y_acceleration: f64,
    color: i32,
}

impl Ball {
    fn new(color: i32) -> Ball {
        Ball {
            x_speed: 5.0,
            y_speed: 2.0,
            radius: 10.0,
            x: 15.0,
            y: 15.0,
            x_acceleration: 1.0,
            y_acceleration: 1.0,
            color,
        }
    }

    fn set_ball(&mut self) {
        self.x = wasm_rng().gen_range(0.0, window_width());
        self.y = wasm_rng().gen_range(0.0, window_width());
    }

    fn move_ball(&mut self) {
        self.x_acceleration = self.x_acceleration + self.x_speed * wasm_rng().gen_range(0.0, window_width());
        self.y_acceleration = self.y_acceleration + self.y_speed * wasm_rng().gen_range(0.0, window_height());

        self.x = self.x + self.x_acceleration;
        self.y = self.y + self.y_acceleration;

        if self.x > window_width() - self.radius || self.x < self.radius {
            self.x_acceleration = self.x_acceleration * -1.0;
        }

        if self.y > window_height() - self.radius || self.y < self.radius {
            self.y_acceleration = self.y_acceleration * -1.0;
        }
    }
}

// fn random_color() -> i32 {
//     let colors = [0x5B7373, 0x393043, 0x662D3F, 0x8F3C5A, 0xB25C66, 0xE09E8F];
//     let mut rng = wasm_rng();
//     let color = wasm_rng().choose(&colors).unwrap();
//     return color;
// }

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn window_width() -> f64 {
    window().inner_width().unwrap().as_f64().unwrap()
}

fn window_height() -> f64 {
    window().inner_height().unwrap().as_f64().unwrap()
}

#[wasm_bindgen]
pub fn draw() {
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut balls: Vec<Ball> = Vec::new(); 
    for _n in 0..50 {
        balls.push(Ball::new(0x5B7373));
    }

    context.clear_rect(0.0, 0.0, window_width(), window_height());
    for b in &mut balls {
        b.set_ball();
    }

    for b in &mut balls {
        context.set_fill_style(&JsValue::from("#BADA55"));
        context.begin_path();
        context.arc(b.x, b.y, b.radius, 0.0, f64::consts::PI * 2.0).unwrap();
        context.fill();
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_rect(0.0, 0.0, window_width(), window_height());
        for b in &mut balls {
            context.set_fill_style(&JsValue::from("#BADA55"));
            context.begin_path();
            context.arc(b.x, b.y, b.radius, 0.0, f64::consts::PI * 2.0).unwrap();
            context.fill();
        }
        for mut b in &mut balls {
            b.move_ball();
            console::log_4(&"Coordinates: ".into(), &b.x.into(), &",".into(), &b.y.into());
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
