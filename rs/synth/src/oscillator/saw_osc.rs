
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::{GlicolNodeData, mono_node};

pub struct SawOsc {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<128>,
    sr: usize,
}

impl SawOsc {
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

impl Node<128> for SawOsc {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        match inputs.len() {
            0 => {
                for i in 0..128 {
                    let period = self.sr as f32 / self.freq;
                    output[0][i] = ( self.phase_n % period as usize) as f32
                    / period *2.0-1.0;
                    self.phase_n += 1;
                }
            },
            1 => {
                for i in 0..128 {
                    let mod_buf = &mut inputs[0].buffers();
                    if mod_buf[0][i] != 0.0 {
                        self.freq = mod_buf[0][i];
                    };
                    let period = self.sr as f32 / self.freq;
                    output[0][i] = ( self.phase_n % period as usize) as f32
                    / period *2.0-1.0;
                    self.phase_n += 1;
                }
            },
            2 => {
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