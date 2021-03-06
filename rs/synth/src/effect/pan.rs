use dasp_graph::{Buffer, Input, Node};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, mono_node};

pub struct Pan {
    pan: f32
}

impl Pan {
    pub fn new(pan: f32) -> GlicolNodeData {
        mono_node!( Self { pan } )
    }
}

impl Node<128> for Pan {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        
        if false {
            assert!(inputs.len() > 0);
            let mod_buf = &mut inputs[0].buffers();

            match inputs[0].buffers().len() {
                1 => {
                    output[0] = inputs[1].buffers()[0].clone();
                    output[1] = inputs[1].buffers()[0].clone();
                },
                2 => {
                    output[0] = inputs[1].buffers()[0].clone();
                    output[1] = inputs[1].buffers()[1].clone();
                },
                _ => {unimplemented!()}
            };
            
            for i in 0..128 {
                let p = mod_buf[0][i];
                output[0][i] *= 1.0 - (p+1.)/2.;
                output[1][i] *= (p+1.)/2.;
            }
            
        } else {
            match inputs[0].buffers().len() {
                1 => {
                    let mut l = inputs[0].buffers()[0].clone();
                    let mut r = l.clone();
                    l.iter_mut().for_each(|s| *s = *s * (1.0 -(self.pan+1./2.)) );
                    r.iter_mut().for_each(|s| *s = *s * (self.pan+1./2.));
                    output[0] = l;
                    output[1] = r;
                },
                2 => {
                    output[0] = inputs[0].buffers()[0].clone();
                    output[1] = inputs[0].buffers()[1].clone();
                    output[0].iter_mut().for_each(|s| *s = *s * (1.0 -(self.pan+1./2.)));
                    output[1].iter_mut().for_each(|s| *s = *s * (self.pan+1./2.));
                },
                _ => {panic!()}
            }
        }
    }
}

#[macro_export]
macro_rules! pan {
    () => { // controlled by modulator, no need for value
        Pan::new(0.5)
    };

    ($data: expr) => {
        Pan::new($data)
    };
}