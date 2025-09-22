use aule::prelude::{AsInput, Input, Signal};
use std::{f32::consts::PI, time::Duration};

pub struct Step {
    amplitude: f32,
}

impl Step {
    pub fn new(amplitude: f32) -> Self {
        Step { amplitude }
    }
}

impl Input for Step {
    fn output(&mut self, dt: Duration) -> Signal {
        Signal {
            value: self.amplitude,
            dt,
        }
    }
}

pub struct Sinusoidal {
    period: f32,
    amplitude: f32,
    offset: f32,
    time: Duration,
}

impl Sinusoidal {
    pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
        Sinusoidal {
            amplitude,
            period,
            offset,
            time: Duration::ZERO,
        }
    }
}

impl Input for Sinusoidal {
    fn output(&mut self, dt: Duration) -> Signal {
        self.time += dt;
        let t = self.time.as_secs_f32();

        Signal {
            value: self.offset + self.amplitude * (2.0 * PI * t / self.period).sin(),
            dt,
        }
    }
}

pub struct Square {
    period: f32,
    amplitude: f32,
    offset: f32,
    time: Duration,
}

impl Square {
    pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
        Square {
            amplitude,
            period,
            offset,
            time: Duration::ZERO,
        }
    }
}

impl Input for Square {
    fn output(&mut self, dt: Duration) -> Signal {
        self.time += dt;
        let t = self.time.as_secs_f32();

        let phase = (t % self.period) / self.period;
        let value = if phase < 0.5 {
            self.offset + self.amplitude
        } else {
            self.offset
        };

        Signal { value, dt }
    }
}

pub struct Sawtooth {
    period: f32,
    amplitude: f32,
    offset: f32,
    time: Duration,
}

impl Sawtooth {
    pub fn new(period: f32, amplitude: f32, offset: f32) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
            time: Duration::ZERO,
        }
    }
}

impl Input for Sawtooth {
    fn output(&mut self, dt: Duration) -> Signal {
        self.time += dt;
        let t = self.time.as_secs_f32();

        let phase = (t % self.period) / self.period;
        let value = self.offset + self.amplitude * phase;

        Signal { value, dt }
    }
}

pub struct Random {
    max_amplitude: f32,
    min_amplitude: f32,
    current_amplitude: Option<f32>,
    max_period: f32,
    min_period: f32,
    current_period: Option<f32>,
    time: Duration,
}

impl Random {
    pub fn new(min_amplitude: f32, max_amplitude: f32, min_period: f32, max_period: f32) -> Self {
        Random {
            max_amplitude,
            min_amplitude,
            current_amplitude: None,
            max_period,
            min_period,
            current_period: None,
            time: Duration::ZERO,
        }
    }

    fn new_amplitude(&self) -> f32 {
        rand::random::<f32>() * (self.max_amplitude - self.min_amplitude) + self.min_amplitude
    }

    fn new_period(&self) -> f32 {
        rand::random::<f32>() * (self.max_period - self.min_period) + self.min_period
    }
}

impl Input for Random {
    fn output(&mut self, dt: Duration) -> Signal {
        self.time += dt;
        let t = self.time.as_secs_f32();

        let mut amplitude = *self.current_amplitude.get_or_insert(self.new_amplitude());
        let period = *self.current_period.get_or_insert(self.new_period());

        let mut phase = (t % period) / period;
        if phase >= 1.0 {
            amplitude = *self.current_amplitude.insert(self.new_amplitude());
            self.current_period = Some(self.new_period());
            phase = 0.0;
        }

        Signal {
            value: amplitude * phase,
            dt,
        }
    }
}
