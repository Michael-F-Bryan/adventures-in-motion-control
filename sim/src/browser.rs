use aimc_comms::Tx;
use aimc_fps_counter::{Fps, FpsSink};
use arrayvec::ArrayString;
use core::fmt::Write;
use js_sys::{Function, Uint8Array};
use wasm_bindgen::JsValue;
use web_sys::Element;

#[derive(Debug, Clone)]
pub struct Browser {
    fps_div: Element,
    tx: Option<Function>,
}

impl Browser {
    pub fn from_element(fps_selector: &str) -> Result<Browser, &'static str> {
        let document = web_sys::window()
            .ok_or("Can't get a reference to the window")?
            .document()
            .ok_or("Can't get a reference to the document")?;

        let element = document
            .query_selector(fps_selector)
            .map_err(|_| "Invalid selector")?
            .ok_or("Can't find the FPS element")?;

        Ok(Browser {
            fps_div: element,
            tx: None,
        })
    }

    pub(crate) fn set_data_sent(&mut self, callback: Function) {
        self.tx = Some(callback);
    }
}

impl FpsSink for Browser {
    fn emit_fps(&mut self, fps: Fps) {
        let mut buffer = ArrayString::<[u8; 128]>::default();

        let result = write!(
            buffer,
            "FPS: {:.1}Hz ({:.1?})",
            fps.frequency, fps.tick_duration
        );

        if result.is_ok() {
            self.fps_div.set_inner_html(&buffer);
        } else {
            self.fps_div.set_inner_html("FPS: ? Hz");
        }
    }
}

impl Tx for Browser {
    fn send(&mut self, data: &[u8]) {
        if let Some(ref tx) = self.tx {
            // efficiently create a typed array directly from WASM memory
            let buffer = Uint8Array::from(data);

            // then try to invoke the callback
            let outcome = tx.call1(&JsValue::NULL, &buffer);

            if let Err(e) = outcome {
                let msg =
                    JsValue::from("An exception was thrown while sending data");
                web_sys::console::error_2(&msg, &e);
            }
        }
    }
}
