
pub mod instruments;
pub mod modules;
pub mod oscillators;

pub const SAMPLE_RATE: u16 = 44100;

fn main() {
    const RIFF: &[u8; 4] = b"RIFF";
    const WAVE: &[u8; 4] = b"WAVE";
    const FMT_: &[u8; 4] = b"fmt ";
    const DATA: &[u8; 4] = b"data";
    let mut sample_data = vec![];
    //// C418 - Wait
    // let inst = Instrument {
    //     op_tree: Operator::Modulated {
    //         carrier: sine,
    //         modulator: Box::new(Operator::Modulated {
    //             carrier: sine,
    //             modulator: Box::new(Operator::Wave {
    //                 oscillator: sine,
    //                 pitch_factor: Envelope::v(4.15),
    //                 level: Envelope::v(9.4),
    //             }),
    //             pitch_factor: Envelope::v(0.25),
    //             level: Envelope::v(0.0005),
    //         }),
    //         level: Envelope::v(1.0),
    //         pitch_factor: Envelope {
    //             levels_durations: vec![(4.0, 0.0), (1.0, 0.001), (0.99, 0.249)],
    //         },
    //     },
    //     envelope: Envelope {
    //         levels_durations: vec![(0.75, 0.0), (1.0, 0.005), (0.2, 0.1405), (0.0, 0.1)],
    //     },
    // };
    // let inst_bis = Instrument {
    //     op_tree: Operator::Modulated {
    //         carrier: sine,
    //         modulator: Box::new(Operator::Modulated {
    //             carrier: sine,
    //             modulator: Box::new(Operator::Wave {
    //                 oscillator: sine,
    //                 pitch_factor: Envelope::v(4.15),
    //                                 level: Envelope::v(9.4),
    //             }),
    //             pitch_factor: Envelope::v(0.25),
    //                             level: Envelope::v(0.0005),
    //         }),
    //         level: Envelope::v(1.0),
    //         pitch_factor: Envelope {
    //             levels_durations: vec![(4.0, 0.0), (1.0, 0.001), (0.99, 0.124)],
    //         },
    //     },
    //     envelope: Envelope {
    //         levels_durations: vec![(0.75, 0.0), (1.0, 0.005), (0.2, 0.0495), (0.0, 0.075)],
    //     },
    // };
    // let inst2 = Instrument {
    //     op_tree: Operator::Modulated {
    //         carrier: sine,
    //         modulator: Box::new(Operator::Modulated {
    //             carrier: sine,
    //             modulator: Box::new(Operator::Wave {
    //                 oscillator: sine,
    //                 pitch_factor: Envelope::v(1.15),
    //                 level: Envelope::v(8.4),
    //             }),
    //             pitch_factor: Envelope::v(4.0),
    //             level: Envelope::v(0.005),
    //         }),
    //         level: Envelope::v(1.0),
    //         pitch_factor: Envelope {
    //             levels_durations: vec![(4.0, 0.0), (1.0, 0.001)],
    //         },
    //     },
    //     envelope: Envelope {
    //         levels_durations: vec![(1.0, 0.0), (0.0, 0.025)],
    //     },
    // };
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst2.play(770.0, t as f32 / SAMPLE_RATE as f32, 0.25) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst_bis.play(400.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst_bis.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut (((inst_bis.play(400.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16).saturating_add((inst2.play(770.0, t as f32 / SAMPLE_RATE as f32, 0.25) * 32767.0) as i16))
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst_bis.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst_bis.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst2.play(770.0, t as f32 / SAMPLE_RATE as f32, 0.25) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst_bis.play(400.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst_bis.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst.play(200.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst2.play(770.0, t as f32 / SAMPLE_RATE as f32, 0.25) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst_bis.play(800.0, t as f32 / SAMPLE_RATE as f32, 0.75) * 32767.0) as i16)
    //         .to_le_bytes()
    //         .to_vec(),
    //     );
    // }
    //// Mike Oldfield's Single
    // let inst = Instrument {
    //     op_tree: Operator::Sum {
    //         ops: vec![
    //             Operator::Modulated {
    //                 carrier: squished_sine,
    //                 modulator: Box::new(Operator::Wave {
    //                     oscillator: sine,
    //                     level: Envelope(vec![(0.1,0.0),(0.1,0.125),(0.5,1.0)]),
    //                                     pitch_factor: Envelope::v(0.5),
    //                 }),
    //                 level: Envelope::v(1.0),
    //                 pitch_factor: Envelope::v(1.0),
    //             },
    //         ],
    //         level: Envelope(vec![(0.5, 0.0), (1.0, 0.125), (0.5, 1.0)]),
    //     },
    //     envelope: Envelope::v(1.0),
    // };
    // for t in 0..SAMPLE_RATE as u32 / 2 {
    //     sample_data.append(
    //         &mut ((inst.play(440.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst.play(494.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 2 {
    //     sample_data.append(
    //         &mut ((inst.play(523.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst.play(587.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 2 {
    //     sample_data.append(
    //         &mut ((inst.play(660.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 4 {
    //     sample_data.append(
    //         &mut ((inst.play(587.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 8 {
    //     sample_data.append(
    //         &mut ((inst.play(660.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    // for t in 0..SAMPLE_RATE as u32 / 2 {
    //     sample_data.append(
    //         &mut ((inst.play(784.0, t as f32 / SAMPLE_RATE as f32, 1.0) * 32767.0) as i16)
    //             .to_le_bytes()
    //             .to_vec(),
    //     );
    // }
    let mut output_bytes = vec![];
    output_bytes.append(&mut RIFF.to_vec());
    output_bytes.append(&mut (sample_data.len() as u32 - 36).to_le_bytes().to_vec());
    output_bytes.append(&mut WAVE.to_vec());
    output_bytes.append(&mut FMT_.to_vec());
    output_bytes.append(&mut 0x10u32.to_le_bytes().to_vec());
    output_bytes.append(&mut 1u16.to_le_bytes().to_vec());
    output_bytes.append(&mut 1u16.to_le_bytes().to_vec());
    output_bytes.append(&mut (SAMPLE_RATE as u32).to_le_bytes().to_vec());
    output_bytes.append(&mut (SAMPLE_RATE as u32 * 2).to_le_bytes().to_vec());
    output_bytes.append(&mut 2u16.to_le_bytes().to_vec());
    output_bytes.append(&mut 16u16.to_le_bytes().to_vec());
    output_bytes.append(&mut DATA.to_vec());
    output_bytes.append(&mut sample_data.len().to_le_bytes().to_vec());
    output_bytes.append(&mut sample_data);
    std::fs::write("tesesest.wav", output_bytes).unwrap();
}
