extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate wbg_rand;

use wbg_rand::{Rng, wasm_rng};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Copy, Clone)]
struct Ball {
    x_speed: f64,
    y_speed: f64,
    radius: f64,
    ratio: i32,
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
            ratio: 1,
            x: 15.0,
            y: 15.0,
            x_acceleration: 1.0,
            y_acceleration: 1.0,
            color,
        }
    }
}

// fn random_color() -> i32 {
//     let colors = [0x5B7373, 0x393043, 0x662D3F, 0x8F3C5A, 0xB25C66, 0xE09E8F];
//     let mut rng = wasm_rng();
//     let color = wasm_rng().choose(&colors).unwrap();
//     return color;
// }

#[wasm_bindgen]
pub struct ClosureHandle(Closure<FnMut()>);

#[wasm_bindgen]
pub fn draw() {
    let window = web_sys::window().unwrap();
    let window_width = window.inner_width().unwrap().as_f64().unwrap();
    let window_height = window.inner_height().unwrap().as_f64().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
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

    let rand_num = || wasm_rng().gen_range(0.0, window_height);;
    let draw = || {
        context.clear_rect(0.0, 0.0, window_width, window_height);
        for b in balls {
            // b.x = wasm_rng().gen_range(0.0, window_width);
            // b.y = wasm_rng().gen_range(0.0, window_height);
            context.set_fill_style(&JsValue::from(b.color));
            context.begin_path();
            context.arc(b.x, b.y, b.radius, 0.0, f64::consts::PI * 2.0).unwrap();
            context.fill();
        }
    };

    draw();

    let _move_balls = || {
        for mut b in balls {
            b.x_acceleration += b.x_speed * wasm_rng().gen_range(0.0, window_width);
            b.y_acceleration += b.y_speed * wasm_rng().gen_range(0.0, window_height);
            b.x += b.x_acceleration;
            b.y += b.y_acceleration;

            if b.x > window_width - b.radius || b.x < b.radius {
                b.x_acceleration = b.x_acceleration * -1.0;
            }

            if b.y > window_height - b.radius || b.y < b.radius {
                b.y_acceleration = b.y_acceleration * -1.0;
            }
        }
    };


    // let animate = Closure::wrap(Box::new(move || {

    // }) as Box<FnMut()>);

    // window.request_animation_frame(animate.as_ref().unchecked_ref());

    // ClosureHandle(animate);
}
