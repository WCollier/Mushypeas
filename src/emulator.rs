use crate::cpu::{Cpu, NUM_KEYS, SCREEN_HEIGHT, SCREEN_WIDTH};

use minifb::{Key, Result, Window, WindowOptions, KeyRepeat};
use std::time::{Instant, Duration};

const WINDOW_WIDTH: usize = SCREEN_WIDTH * 10;

const WINDOW_HEIGHT: usize = SCREEN_HEIGHT * 10;

static KEYS: [(Key, usize); NUM_KEYS] = [
    (Key::Key1, 0x1),
    (Key::Key2, 0x2),
    (Key::Key3, 0x3),
    (Key::Key4, 0xC),

    (Key::Q, 0x4),
    (Key::W, 0x5),
    (Key::E, 0x6),
    (Key::R, 0xD),

    (Key::A, 0x7),
    (Key::S, 0x8),
    (Key::D, 0x9),
    (Key::F, 0xE),

    (Key::Z, 0xA),
    (Key::X, 0x0),
    (Key::C, 0xB),
    (Key::V, 0xF),
];

pub(crate) struct Emulator {
    cpu: Cpu,
    window: Window,
    screen_buffer: Vec<u32>,
}

impl Emulator {
    pub(crate) fn new() -> Self {
        let mut window = Window::new(
            "Mushypeas",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default(),
        )
        .expect("Could not create window");

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Emulator {
            cpu: Cpu::new(),
            window,
            screen_buffer: vec![0x0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub(crate) fn run(&mut self, raw: &[u8]) {
        self.cpu.load(raw);

        let mut instr_time = Instant::now();

        let mut draw_time = Instant::now();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();

            if instr_time.elapsed() > Duration::from_millis(1) {
                self.cpu.run();

                instr_time = Instant::now();
            }

            if draw_time.elapsed() > Duration::from_millis(10) {
                self.draw();

                draw_time = Instant::now();
            }
        }
    }

    fn update(&mut self) {
        for (key, i) in &KEYS {
            self.cpu.keys[*i] = self.window.is_key_down(*key);
        }
    }

    fn draw(&mut self) {
        if !self.cpu.should_rerender {
            return;
        }

        for (i, pixel) in self.screen_buffer.iter_mut().enumerate() {
            *pixel = if self.cpu.display[i] { 0xFFFFFF } else { 0 };
        }

        self.window
            .update_with_buffer(&self.screen_buffer, SCREEN_WIDTH, SCREEN_HEIGHT);
    }
}
