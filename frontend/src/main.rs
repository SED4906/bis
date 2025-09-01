use std::time::Duration;

use sdl3::{
    audio::{AudioCallback, AudioFormat, AudioFormatNum, AudioSpec, AudioStream},
    event::Event,
    gpu::ShaderFormat,
    keyboard::Keycode,
    pixels::Color,
};
use synth::{
    instruments::{Envelope, Instrument, Operator}, oscillators::Oscillator,
};

const FPS_LIMIT: Duration = Duration::new(0, 1_000_000_000u32 / 60);
const DEBUG_MODE: bool = true;

pub struct AudioSynth {
    pub frequency: f32,
    pub t: i64,
    pub phase: f32,
    pub instrument: Instrument,
}

impl<Channel> AudioCallback<Channel> for AudioSynth
where
    Channel: AudioFormatNum,
{
    fn callback(&mut self, out: &mut AudioStream, requested: i32) {
        for _ in 0..requested {
            out.put_data_i16(&[(self
                .instrument
                .play(self.frequency, self.t as f32 / 44100.0, 1.0)
                * 32767.0) as i16]).unwrap();
            self.t += 1;
        }
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("bis", 512, 512)
        .position_centered()
        .borderless()
        .build()
        .unwrap();
    let gpu_device = sdl3::gpu::Device::new(ShaderFormat::SPIRV, DEBUG_MODE)
        .unwrap()
        .with_window(&window)
        .unwrap();

    let audio_spec = AudioSpec::new(Some(44100), Some(1), Some(AudioFormat::S16LE));
    let audio_device = audio_subsystem.open_playback_device(&audio_spec).unwrap();

    let audio_synth = AudioSynth {
        frequency: 440.0,
        t: 0,
        phase: 0.0,
        instrument: Instrument {
            op_tree: Operator::Wave {
                oscillator: Oscillator::Sine,
                level: Envelope(vec![(1.0,0.0), (0.0,1.0)]),
                pitch_factor: Envelope::v(1.0),
            },
            envelope: Envelope::v(1.0),
        },
    };

    let mut audio_playback = audio_subsystem
        .open_playback_stream_with_callback::<AudioSynth, i16>(
            &audio_device,
            &audio_spec,
            audio_synth,
        )
        .unwrap();

    audio_playback.resume().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    audio_playback.lock().unwrap().t = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let mut lock_guard = audio_playback.lock().unwrap();
                    lock_guard.frequency *= 1.0594631;
                    lock_guard.t = ((lock_guard.t as f32 / 1.0594631) % (44100.0 / lock_guard.frequency)) as i64;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let mut lock_guard = audio_playback.lock().unwrap();
                    lock_guard.frequency /= 1.0594631;
                    lock_guard.t = ((lock_guard.t as f32 * 1.0594631) % (44100.0 / lock_guard.frequency)) as i64;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    println!("{} Hz",audio_playback.lock().unwrap().frequency);
                    println!("{}",audio_playback.lock().unwrap().instrument);
                }
                _ => {}
            }
        }
        ::std::thread::sleep(FPS_LIMIT);
    }
}
