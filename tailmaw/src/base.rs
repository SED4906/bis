use sdl3::{AudioSubsystem, Sdl, VideoSubsystem, gpu::{Device, ShaderFormat}, video::Window};

use crate::hammerspace::Hammerspace;

pub struct Engine<'a> {
    sdl: Sdl,
    video: VideoSubsystem,
    audio: AudioSubsystem,
    window: Window,
    device: Device,
    assets: Hammerspace<'a>,
}

const DEBUG_MODE: bool = true;

impl Engine<'_> {
    pub fn new() -> Self {
        let sdl = sdl3::init().expect("sdl init failed");
        let video = sdl.video().expect("sdl video failed");
        let audio = sdl.audio().expect("sdl audio failed");
        let window = video.window("bis game engine", 640, 360).position_centered().build().expect("sdl window failed");
        let device = Device::new(ShaderFormat::SPIRV, DEBUG_MODE).expect("gpu init failed").with_window(&window).expect("gpu window failed");
        let assets = Hammerspace::new();
        Self {
            sdl,
            video,
            audio,
            window,
            device,
            assets,
        }
    }

    pub fn retitle_window(&mut self, title: &str) {
        self.window.set_title(title).expect("failed to set window title");
    }
}

impl Default for Engine<'_> {
    fn default() -> Self {
        Self::new()
    }
}
