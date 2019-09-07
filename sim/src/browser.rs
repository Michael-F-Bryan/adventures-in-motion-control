use anpp::Packet;
use arrayvec::ArrayString;
use comms::Tx;
use core::fmt::Write;
use fps_counter::{Fps, FpsSink};
use web_sys::Element;

#[derive(Debug, Clone)]
pub struct Browser {
    fps_div: Element,
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

        Ok(Browser { fps_div: element })
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
    fn send(&mut self, _packet: Packet) { unimplemented!() }
}
