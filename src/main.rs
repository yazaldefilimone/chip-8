mod utils;
use rand::Rng;
use std::fs::File;
use std::io::Read;
use utils::{FONT_SET, FONT_SET_SIZE, FONT_START_ADDRESS, START_ADDRESS};

enum Instruction {
  CLS, // 00E0
  RET, // 00EE
}

struct ChipEighth {
  registers: Vec<u8>,
  memory: Vec<u8>,
  index: usize,
  program_index: usize,
  stack_index: usize,
  stack: Vec<usize>,
  delay_timer: u8,
  sound_timer: u8,
  keypad: Vec<u8>,
  video_ram: Vec<u32>,
  opcode_table: Vec<u8>,
}
// create global variables, start_address, end_address

impl ChipEighth {
  fn new() -> ChipEighth {
    ChipEighth {
      registers: [0; 8].to_vec(),
      memory: [0; 4096].to_vec(),
      index: 0,
      program_index: START_ADDRESS,
      stack_index: 0,
      stack: vec![].to_vec(),
      delay_timer: 0,
      sound_timer: 0,
      keypad: [0; 8].to_vec(),
      video_ram: [0; 256].to_vec(),
      opcode_table: [0; 256].to_vec(),
    }
  }

  fn load_font(&mut self) {
    for i in 0..FONT_SET_SIZE {
      self.memory[FONT_START_ADDRESS + i] = FONT_SET[i];
    }
  }

  fn random(&mut self) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=255)
  }
  fn load_rom(&mut self, filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let mut index = 0;
    for character in buffer.chars() {
      self.memory[index] = character as u8;
      index += 1;
    }
  }

  fn inst_cls(&mut self) {
    // We can simply set the entire video buffer to zeroes.
    let size = self.video_ram.len();
    for i in 0..size {
      self.video_ram[i] = 0;
    }
  }

  fn inst_ret(&mut self) {
    self.stack_index -= 1;
    self.index = self.stack[self.stack_index];
  }
}

fn main() {
  println!("Hello, world!");
}
