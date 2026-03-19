use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, GainNode};

pub struct AudioManager {
    ctx: AudioContext,
    master_gain: GainNode,
}

impl AudioManager {
    pub fn new() -> Result<Self, JsValue> {
        let ctx = AudioContext::new()?;
        let master_gain = ctx.create_gain()?;
        master_gain.connect_with_audio_node(&ctx.destination())?;
        master_gain.gain().set_value(0.5);

        Ok(Self { ctx, master_gain })
    }

    pub async fn load_sound(&self, url: &str) -> Result<web_sys::AudioBuffer, JsValue> {
        let response = JsFuture::from(web_sys::window().unwrap().fetch_with_str(url)).await?;
        let response: web_sys::Response = response.dyn_into()?;
        let array_buffer = JsFuture::from(response.array_buffer()?).await?;
        let buffer = JsFuture::from(self.ctx.decode_audio_data(&array_buffer.into())?).await?;
        Ok(buffer.dyn_into()?)
    }

    pub fn play_loop(&self, buffer: &web_sys::AudioBuffer) -> Result<web_sys::AudioBufferSourceNode, JsValue> {
        let source = self.ctx.create_buffer_source()?;
        source.set_buffer(Some(buffer));
        source.set_loop(true);
        source.connect_with_audio_node(&self.master_gain)?;
        source.start()?;
        Ok(source)
    }

    pub fn play_once(&self, buffer: &web_sys::AudioBuffer) -> Result<(), JsValue> {
        let source = self.ctx.create_buffer_source()?;
        source.set_buffer(Some(buffer));
        source.connect_with_audio_node(&self.master_gain)?;
        source.start()?;
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) {
        self.master_gain.gain().set_value(volume.clamp(0.0, 1.0));
    }

    pub fn resume(&self) -> Result<js_sys::Promise, JsValue> {
        self.ctx.resume()
    }
}
