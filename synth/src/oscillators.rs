fn sine(t: f32) -> f32 {
    (core::f32::consts::PI * 2.0 * t).sin()
}

fn sawtooth(t: f32) -> f32 {
    (t % 1.0) * 2.0 - 1.0
}

fn square(t: f32) -> f32 {
    ((t * 2.0).floor() % 2.0) * 2.0 - 1.0
}

fn abs_sine(t: f32) -> f32 {
    sine(t).abs()
}

fn squished_sine(t: f32) -> f32 {
    0.5 * sine(t*2.0) * (1.0 - square(t))
}

fn squished_abs_sine(t: f32) -> f32 {
    0.5 * abs_sine(t*2.0) * (1.0 - square(t))
}

fn half_sine(t: f32) -> f32 {
    0.5 * sine(t) * (1.0 - square(t))
}

fn quarter_sine(t: f32) -> f32 {
    0.5 * sine(t) * (1.0 - square(t * 2.0))
}

pub enum Oscillator {
    Sine,
    Sawtooth,
    Square,
    AbsSine,
    SquishedSine,
    SquishedAbsSine,
    HalfSine,
    QuarterSine
}

impl Oscillator {
    pub fn sample(&self, t: f32) -> f32 {
        match self {
            Oscillator::Sine => sine(t),
            Oscillator::Sawtooth => sawtooth(t),
            Oscillator::Square => square(t),
            Oscillator::AbsSine => abs_sine(t),
            Oscillator::SquishedSine => squished_sine(t),
            Oscillator::SquishedAbsSine => squished_abs_sine(t),
            Oscillator::HalfSine => half_sine(t),
            Oscillator::QuarterSine => quarter_sine(t),
        }
    }
}

impl core::fmt::Display for Oscillator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Oscillator::Sine => f.write_str("sine")?,
            Oscillator::Sawtooth => f.write_str("sawtooth")?,
            Oscillator::Square => f.write_str("square")?,
            Oscillator::AbsSine => f.write_str("abs-sine")?,
            Oscillator::SquishedSine => f.write_str("squished-sine")?,
            Oscillator::SquishedAbsSine => f.write_str("squished-abs-sine")?,
            Oscillator::HalfSine => f.write_str("half-sine")?,
            Oscillator::QuarterSine => f.write_str("quarter-sine")?,
        }
        Ok(())
    }
}
