use failure::Error;
use fps_counter::{Fps, FpsSink};
use wasm_bindgen::JsValue;
use web_sys::Element;

#[derive(Debug, Clone)]
pub struct Browser {
    fps_div: Element,
}

impl Browser {
    pub fn from_element(fps_selector: &str) -> Result<Browser, Error> {
        let document = web_sys::window()
            .ok_or_else(|| {
                failure::err_msg("Can't get a reference to the window")
            })?
            .document()
            .ok_or_else(|| {
                failure::err_msg("Can't get a reference to the document")
            })?;

        let element = document
            .query_selector(fps_selector)
            .map_err(|_| failure::err_msg("Invalid selector"))?
            .ok_or_else(|| failure::err_msg("Can't find the FPS element"))?;

        Ok(Browser { fps_div: element })
    }

    pub fn log(&mut self, msg: &str) {
        let msg = JsValue::from(msg);
        web_sys::console::log_1(&msg);
    }
}

impl FpsSink for Browser {
    fn emit_fps(&mut self, fps: Fps) {
        let label = format!("FPS: {:.2}Hz", fps.frequency);
        self.fps_div.set_inner_html(&label);
    }
}
