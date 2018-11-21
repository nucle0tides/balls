extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

fn random_color() -> String {
    let colors = ["#5B7373", "#393043", "#662D3F", "#8F3C5A", "#B25C66", "#E09E8F"];
    return String::from("#E09E8F")
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

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
