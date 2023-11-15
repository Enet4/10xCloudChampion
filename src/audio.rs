//! Audio module

use js_sys::wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::HtmlAudioElement;

use crate::central::state::try_local_storage;

pub static BUTTON_OP_CLICK: &str = "assets/audio/opclick.ogg";
pub static BUTTON_ZIP_CLICK: &str = "assets/audio/zipclick.ogg";

pub fn play_op_click() {
    play(BUTTON_OP_CLICK, 0.1);
}

pub fn play_zip_click() {
    play(BUTTON_ZIP_CLICK, 0.25);
}

pub fn play(file_path: &str, volume: f64) {
    match is_enabled() {
        Ok(true) => {
            let audio_elem = HtmlAudioElement::new_with_src(file_path).unwrap_throw();
            audio_elem.set_cross_origin(Some("anonymous"));
            audio_elem.set_volume(volume);
            let _ = audio_elem.play();
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
