extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate wbg_rand;

use wbg_rand::{Rng, wasm_rng};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct Ball {
    x_speed: f64,
    y_speed: f64,
    radius: f64,
    ratio: i32,
    x: f64,
    y: f64,
    x_acceleration: f64,
    y_acceleration: f64,
    color: String,
}

impl Ball {
    fn new(color: String) -> Ball {
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

// fn random_color() -> Option<&String> {
//     let colors: Vec<String> = vec![String::from("#5B7373"), String::from("#393043"), String::from("#662D3F"), String::from("#8F3C5A"), String::from("#B25C66"), String::from("#E09E8F")];
//     let mut rng = thread_rng();
//     let color = colors.choose(&mut rng);
//     return color
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
        balls.push(Ball::new(String::from("#5a5ca7")));
    }

    let rand_num = || wasm_rng().gen_range(0.0, window_height);;
    let draw = || {
        // context.clear_rect(0.0, 0.0, window_width, window_height);
        for mut b in balls {
            b.x = wasm_rng().gen_range(0.0, window_width);
            b.y = wasm_rng().gen_range(0.0, window_height);
            context.set_fill_style(&JsValue::from(b.color));
            context.begin_path();
            context.arc(b.x, b.y, b.radius, 0.0, f64::consts::PI * 2.0).unwrap();
            context.fill();
        }
    };

    draw();

    // let _move_balls = || {
    //     for mut b in balls {
    //         b.x_acceleration += b.x_speed * rand::random::<f64>();
    //         b.y_acceleration += b.y_speed * rand::random::<f64>();
    //         b.x += b.x_acceleration;
    //         b.y += b.y_acceleration;

    //         if b.x > window_width - b.radius || b.x < b.radius {
    //             b.x_acceleration = b.x_acceleration * -1.0;
    //         }

    //         if b.y > window_height - b.radius || b.y < b.radius {
    //             b.y_acceleration = b.y_acceleration * -1.0;
    //         }
    //     }
    // };


    // let animate = Closure::wrap(Box::new(move || {

    // }) as Box<FnMut()>);

    // window.request_animation_frame(animate.as_ref().unchecked_ref());

    // ClosureHandle(animate);
}
