
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct TriOsc {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<128>,
    sr: usize,
}

impl TriOsc {
    pub fn new() -> Self {
        Self {
            freq: 0.01,
            phase_n: 0,
            clock: 0,
            buffer: Buffer::<128>::default(),
            sr: 44100,
        }
    }
    pub fn freq(self, freq: f32) -> Self {
        Self {freq, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData {
        mono_node! {
            self
        }
    }
}

impl Node<128> for TriOsc {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        match inputs.len() {
            0 => {
                for i in 0..128 {
                    let t = self.sr as f32 / self.freq;
                    let phase = self.phase_n as f32 / (t/4.);
                    let y = match phase.floor() as u8 {
                        0 => phase.fract(),
                        1 => 1.0 - phase.fract(),
                        2 => - phase.fract(),
                        3 => phase.fract() - 1.,
                        _ => 0.0
                    };
                    // let period = self.sr as f32 / self.freq;
                    // let quater_period = (period / 4.) as usize;
                    // let half_period = (period / 2.) as usize;
                    // let y_abs = (self.phase_n % quater_period) as f32 / quater_period as f32;
                    // let is_quater = ((self.phase_n % half_period) >= quater_period) as u8 as f32;// 1.0 or 0
                    // let is_half =  (self.phase_n >= half_period ) as u8 as f32;// 1.0 or 0
                    // let y = (y_abs * (is_quater * -2. + 1.) + (1. * is_quater)) * (is_half * -2. + 1.);
                    output[0][i] = y;
                    self.phase_n += 1;
                    if self.phase_n >= t as usize {
                        self.phase_n -= t as usize;
                    }
                }
            },
            1 => { // can be clock in Glicol or sidechain made by user
                for i in 0..128 {
                    let mod_buf = &mut inputs[0].buffers();
                    if mod_buf[0][i] != 0.0 {
                        self.freq = mod_buf[0][i];
                    };
                    let t = self.sr as f32 / self.freq;
                    let phase = self.phase_n as f32 / (t/4.);
                    let y = match phase.floor() as u8 {
                        0 => phase.fract(),
                        1 => 1.0 - phase.fract(),
                        2 => - phase.fract(),
                        3 => phase.fract() - 1.,
                        _ => 0.0
                    };
                    output[0][i] = y;
                    self.phase_n += 1;
                    if self.phase_n >= t as usize {
                        self.phase_n -= t as usize;
                    }
                }
            },
            2 => { // clock + sidechain or mistake
                let mut clock = inputs[1].buffers()[0][0] as usize;
                for i in 0..128 {
                    let mod_buf = &mut inputs[0].buffers();
                    if mod_buf[0][i] != 0.0 {
                        self.freq = mod_buf[0][i];
                    };
                    let period = self.sr as f32 / self.freq;
                    output[0][i] = (clock % period as usize) as f32
                    / period *2.0-1.0;
                    clock += 1;
                }
            },
            _ => ()
        }
    }
}