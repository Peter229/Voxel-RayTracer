use crate::input;
use crate::texture;
use crate::camera;
use crate::chunk;

use winit::event::*;

pub struct Game {
    inputs: input::Inputs,
    pub chunk: chunk::Chunk,
}

impl Game {
    pub fn new(device: &wgpu::Device) -> Game {
        let inputs = input::Inputs::new();
        let chunk = chunk::Chunk::new(device);
        Game { inputs, chunk }
    }

    pub fn init(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
   
    }

    pub fn input(&mut self, event: &WindowEvent) {

        self.inputs.input(event);
    }

    pub fn process_inputs(&mut self, w_di: [u32; 2]) {

    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self) {
        
    }
}