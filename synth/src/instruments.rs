use crate::oscillators::Oscillator;

pub struct Envelope(pub Vec<(f32, f32)>);

fn lerp(start: f32, end: f32, d: f32) -> f32 {
    start + (end - start) * d
}

impl Envelope {
    pub fn sample(&self, t: f32) -> f32 {
        let mut t_total = 0.0;
        let mut result = 0.0;
        for (level, duration) in &self.0 {
            t_total += duration;
            if t_total >= t && *duration != 0.0 {
                return lerp(result, *level, 1.0 - (t_total - t) / *duration);
            }
            result = *level;
        }
        result
    }

    pub fn v(level: f32) -> Self {
        Envelope(vec![(level, 0.0)])
    }

    //pub fn parse(input: &str) -> IResult<&str, Self> {}
}

pub enum Operator {
    Modulated {
        carrier: Oscillator,
        modulator: Box<Operator>,
        level: Envelope,
        pitch_factor: Envelope,
    },
    Sum {
        ops: Vec<Operator>,
        level: Envelope,
    },
    Wave {
        oscillator: Oscillator,
        level: Envelope,
        pitch_factor: Envelope,
    },
}

pub struct Instrument {
    pub op_tree: Operator,
    pub envelope: Envelope,
}

impl Operator {
    pub fn sample(&self, frequency: f32, t: f32) -> f32 {
        match self {
            Operator::Modulated {
                carrier,
                modulator,
                level,
                pitch_factor,
            } => {
                carrier.sample(frequency * pitch_factor.sample(t) * t + modulator.sample(frequency, t))
                    * level.sample(t)
            }
            Operator::Sum { ops, level } => {
                ops.iter().fold(0.0, |res, op| {
                    res + op.sample(frequency, t) * level.sample(t)
                }) / ops.len() as f32
            }
            Operator::Wave {
                oscillator,
                level,
                pitch_factor,
            } => oscillator.sample(frequency * pitch_factor.sample(t) * t) * level.sample(t),
        }
    }
}

impl Instrument {
    pub fn play(&self, frequency: f32, t: f32, volume: f32) -> f32 {
        volume * self.op_tree.sample(frequency, t) * self.envelope.sample(t)
    }

    //pub fn parse(input: &str) -> IResult<&str, Self> {}
}

impl core::fmt::Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(instrument {} {})",
            self.op_tree, self.envelope
        ))?;
        Ok(())
    }
}

impl core::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Modulated {
                carrier,
                modulator,
                level,
                pitch_factor,
            } => f.write_fmt(format_args!(
                "(modulated {carrier} {modulator} {level} {pitch_factor})"
            ))?,
            Operator::Sum { ops, level } => {
                f.write_str("(sum (")?;
                if let Some(first_op) = ops.iter().next() {
                    f.write_fmt(format_args!("{first_op}"))?;
                }
                for op in ops.iter().skip(1) {
                    f.write_fmt(format_args!(" {op}"))?;
                }
                f.write_fmt(format_args!(") {level})"))?;
            },
            Operator::Wave {
                oscillator,
                level,
                pitch_factor,
            } => f.write_fmt(format_args!("(wave {oscillator} {level} {pitch_factor})"))?,
        }
        Ok(())
    }
}

impl core::fmt::Display for Envelope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() == 0 {
            f.write_str("0")?;
        } else if self.0.len() == 1 && self.0[0].1 == 0.0 {
            f.write_fmt(format_args!("{}", self.0[0].0))?;
        } else if self.0[0].1 != 0.0 {
            f.write_fmt(format_args!("(0"))?;
            for (level, duration) in self.0.iter() {
                f.write_fmt(format_args!(" ({level} {duration})"))?;
            }
            f.write_fmt(format_args!(")"))?;
        } else {
            f.write_fmt(format_args!("({}", self.0[0].0))?;
            for (level, duration) in self.0.iter().skip(1) {
                f.write_fmt(format_args!(" ({level} {duration})"))?;
            }
            f.write_fmt(format_args!(")"))?;
        }
        Ok(())
    }
}
