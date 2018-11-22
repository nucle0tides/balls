extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate rand;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Ball {
    x_speed: i32,
    y_speed: i32,
    radius: i32,
    ratio: i32,
    x_origin: i32,
    y_origin: i32,
    color: String,
}

impl Ball {
    fn new(color: String) -> Ball {
        Ball {
            x_speed: 5,
            y_speed: 2,
            radius: 10,
            ratio: 1,
            x_origin: 0,
            y_origin: 0,
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

fn setup() {
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

}

#[wasm_bindgen]
pub fn draw() {
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
    for n in 0..50 {
        balls.push(Ball::new(String::from("#526055")));
    }
}
