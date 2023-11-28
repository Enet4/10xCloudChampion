//! Audio module

use js_sys::{
    wasm_bindgen::{JsCast as _, JsValue, UnwrapThrowExt},
    Reflect,
};
use web_sys::HtmlAudioElement;

use crate::central::state::try_local_storage;

pub static BUTTON_OP_CLICK: &str = "assets/audio/opclick.ogg";
pub static BUTTON_ZIP_CLICK: &str = "assets/audio/zipclick.ogg";

fn create_audio_element(path: &str) -> HtmlAudioElement {
    let audio_elem = HtmlAudioElement::new_with_src(path).unwrap_throw();
    audio_elem.set_cross_origin(Some("anonymous"));
    audio_elem
}

/// Get an audio element,
/// creating it only if it was not done before
/// (saving it in global window context).
fn load_audio_once(file_path: &str, property_name: &str) -> HtmlAudioElement {
    if let Some(window) = web_sys::window() {
        let audio_elem = window.get(property_name);
        if let Some(audio_elem) = audio_elem {
            return audio_elem.dyn_into::<HtmlAudioElement>().unwrap_throw();
        } else {
            let audio_elem = create_audio_element(file_path);
            let _ = Reflect::set(&window, &JsValue::from_str(property_name), &audio_elem);
            audio_elem
        }
    } else {
        create_audio_element(BUTTON_OP_CLICK)
    }
}

fn load_op_click() -> HtmlAudioElement {
    load_audio_once(BUTTON_OP_CLICK, "__op_click_audio")
}

fn load_zip_click() -> HtmlAudioElement {
    load_audio_once(BUTTON_ZIP_CLICK, "__zip_click_audio")
}

pub fn play_op_click() {
    play(&load_op_click(), 0.1);
}

pub fn play_zip_click() {
    play(&load_zip_click(), 0.25);
}

pub fn play(elem: &HtmlAudioElement, volume: f64) {
    match is_enabled() {
        Ok(true) => {
            if let Ok(audio_elem) = elem.clone_node() {
                let audio_elem: HtmlAudioElement = audio_elem.dyn_into().unwrap();
                audio_elem.set_volume(volume);
                let _ = audio_elem.play();
            }
        }
        Ok(false) => {}
        Err(e) => {
            gloo_console::error!("Error playing audio:", e);
        }
    }
}

pub fn is_enabled() -> Result<bool, JsValue> {
    let local_storage = try_local_storage()?;
    let out = local_storage.get("audio")?;

    if let Some(enabled) = out {
        Ok(enabled == "true")
    } else {
        local_storage.set("audio", "true")?;
        Ok(true)
    }
}

pub fn set_audio(enabled: bool) -> Result<(), JsValue> {
    let local_storage = try_local_storage()?;
    local_storage.set("audio", if enabled { "true" } else { "false" })?;
    Ok(())
}
