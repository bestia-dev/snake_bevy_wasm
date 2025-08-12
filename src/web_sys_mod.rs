//! src/web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! Trying to isolate/hide all javascript code and conversion here.

#![allow(dead_code)]

// region: use
// the macro unwrap! shows the TRUE location where the error has occurred.
use unwrap::unwrap;
use wasm_bindgen::JsValue;
// use wasm_bindgen_futures::JsFuture;
use web_sys::console;
// use web_sys::{Request, RequestInit, Response};
// endregion: use

/// return the global window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
/// Only correctly html encoded strings can use this function.
/// set inner html into dom
pub fn set_html_element_inner_html(element_id: &str, inner_html: &str) {
    let html_element = get_element_by_id(element_id);
    html_element.set_inner_html(inner_html);
}

// get the width of the viewport
pub fn get_client_width() -> i32 {
    web_sys::window()
        .expect("There should be a window")
        .document()
        .expect("There should be a document")
        .document_element()
        .expect("There should be a document_element")
        .client_width()
}

// get the height of the viewport
pub fn get_client_height() -> i32 {
    web_sys::window()
        .expect("There should be a window")
        .document()
        .expect("There should be a document")
        .document_element()
        .expect("There should be a document_element")
        .client_height()
}

// get scale factor DPR of screen
pub fn get_device_pixel_ratio() -> f32 {
    web_sys::window().expect("There should be a window").device_pixel_ratio() as f32
}
