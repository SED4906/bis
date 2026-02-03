use sdl3::{
    AudioSubsystem, EventPump, Sdl, VideoSubsystem,
    event::Event,
    gpu::{ColorTargetInfo, DepthStencilTargetInfo, Device, LoadOp, ShaderFormat, StoreOp},
    pixels::Color,
    video::Window,
};

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
        let window = video
            .window("bis game engine", 640, 360)
            .position_centered()
            .build()
            .expect("sdl window failed");
        let device = Device::new(ShaderFormat::SPIRV, DEBUG_MODE)
            .expect("gpu init failed")
            .with_window(&window)
            .expect("gpu window failed");
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
        self.window
            .set_title(title)
            .expect("failed to set window title");
    }

    pub fn update(&mut self) -> bool {
        let mut event_pump = self.sdl.event_pump().expect("sdl event pump failed");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return false;
                }
                _ => {}
            }
        }
        self.render();
        true
    }

    fn render(&mut self) {
        let mut command_buffer = self
            .device
            .acquire_command_buffer()
            .expect("acquire command buffer failed");
        let swapchain_texture = command_buffer
            .wait_and_acquire_swapchain_texture(&self.window)
            .expect("swapchain texture failed");
        let color_target_info = ColorTargetInfo::default()
            .with_texture(&swapchain_texture)
            .with_load_op(LoadOp::DONT_CARE)
            .with_store_op(StoreOp::STORE)
            .with_clear_color(Color::RGB(0xbb, 0x11, 0x55));

        let render_pass = self
            .device
            .begin_render_pass(&command_buffer, &[color_target_info], None)
            .expect("begin render pass failed");
        self.device.end_render_pass(render_pass);

        let fence = command_buffer
            .submit_and_acquire_fence(&self.device)
            .expect("submit and acquire fence failed");
        self.device
            .wait_fences(true, &[fence])
            .expect("wait fence failed");
    }
}

impl Default for Engine<'_> {
    fn default() -> Self {
        Self::new()
    }
}
