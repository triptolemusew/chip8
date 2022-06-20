use crate::bus::Bus;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::display::{Color, DisplaySink};

const SIZE_OF_SPRITE_FOR_DIGIT: u16 = 5;

#[derive(Default)]
pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: usize,
    sound_timer: u8,
    delay_timer: u8,
    stack: Vec<usize>,
    is_key_pressed: [bool; 16],
    pub draw_enable: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sound_timer: 0,
            delay_timer: 0,
            stack: Vec::with_capacity(12),
            is_key_pressed: [false; 16],
            draw_enable: true,
        }
    }

    pub fn fetch_execute(&mut self, bus: &mut Bus, display_sink: &mut DisplaySink) {
        let instruction = self.fetch_instruction(bus);
        self.execute_instruction(instruction, bus, display_sink);
    }

    fn fetch_instruction(&mut self, bus: &mut Bus) -> u16 {
        let hi = bus.ram_read_byte(self.pc as u16);
        let lo = bus.ram_read_byte((self.pc + 1) as u16);

        self.pc += 2;
        self.update_timers();

        u16::from_be_bytes([hi, lo])
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 && self.sound_timer == 1 {
            self.sound_timer -= 1;
        }
    }

    fn execute_instruction(
        &mut self,
        instruction: u16,
        bus: &mut Bus,
        display_sink: &mut DisplaySink,
    ) {
        const F: usize = 0xF;
        match instruction & 0xF000 {
            0x0000 => match instruction & 0x0FFF {
                0x00E0 => {
                    bus.clear_screen();
                }
                0x00EE => {
                    if let Some(return_address) = self.stack.pop() {
                        self.pc = return_address;
                    } else {
                        unimplemented!()
                    }
                }
                _ => unreachable!(),
            },
            0x1000 => {
                self.pc = usize::from(instruction & 0x0FFF);
            }
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = usize::from(instruction & 0x0FFF);
            }
            0x3000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                if self.v[x] == (instruction & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                if self.v[x] != (instruction & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                let y = usize::from((instruction & 0x00F0) >> 4);

                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                self.v[x] = (instruction & 0x00FF) as u8;
            }
            0x7000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                self.v[x] = self.v[x].wrapping_add((instruction & 0x00FF) as u8);
            }
            0x8000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                let y = usize::from((instruction & 0x00F0) >> 4);

                match instruction & 0x000F {
                    0x0000 => {
                        self.v[x] = self.v[y];
                    }
                    0x0001 => {
                        self.v[x] |= self.v[y];
                    }
                    0x0002 => {
                        self.v[x] &= self.v[y];
                    }
                    0x0003 => {
                        self.v[x] ^= self.v[y];
                    }
                    0x0004 => {
                        let (result, carry) = self.v[x].overflowing_add(self.v[y]);
                        self.v[x] = result;
                        self.v[F] = carry as u8;
                    }
                    0x0005 => {
                        let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
                        self.v[x] = result;
                        self.v[F] = !borrow as u8;
                    }
                    0x0006 => {
                        self.v[F] = (self.v[y] & 0x01 != 0) as u8;
                        self.v[x] = self.v[y] >> 1;
                    }
                    0x0007 => {
                        let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
                        self.v[x] = result;
                        self.v[F] = !borrow as u8;
                    }
                    0x000E => {
                        self.v[F] = (self.v[y] & 0x80 != 0) as u8;
                        self.v[x] = self.v[y] << 1;
                    }
                    _ => unreachable!(),
                }
            }
            0x9000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                let y = usize::from((instruction & 0x00F0) >> 4);

                match instruction & 0x000F {
                    0x0000 => {
                        if self.v[x] != self.v[y] {
                            self.pc += 2;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            0xA000 => {
                self.i = instruction & 0x0FFF;
            }
            0xB000 => {
                self.pc = usize::from(instruction & 0x0FFF) + usize::from(self.v[0]);
            }
            0xC000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                self.v[x] = rand::random::<u8>() & ((instruction & 0x00FF) as u8);
            }
            0xD000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);
                let vx = usize::from(self.v[x]) % SCREEN_WIDTH;

                let y = usize::from((instruction & 0x00F0) >> 4);
                let vy = usize::from(self.v[y]) % SCREEN_HEIGHT;

                self.v[F] = 0;

                for row in 0..(instruction & 0x000F) {
                    let pixel_y = vy + usize::from(row);
                    if pixel_y >= SCREEN_HEIGHT {
                        break;
                    }
                    for col in 0..8u16 {
                        let pixel_x = vx + usize::from(col);
                        if pixel_x >= SCREEN_WIDTH {
                            break;
                        }
                        if bus.ram_read_byte(self.i + row) & (1 << (7 - col)) != 0 {
                            let pixel = &mut bus.get_display_buffer()[pixel_y][pixel_x];
                            if let Color::White = *pixel {
                                self.v[F] = 1;
                            }
                            *pixel = match *pixel {
                                Color::Black => Color::White,
                                Color::White => Color::Black,
                            };
                        }
                    }
                }
                display_sink.append(bus.get_display_buffer().clone());
            }
            0xE000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);

                match instruction & 0x00FF {
                    0x009E => {
                        // TODO: Proper logic for keyboard
                        if self.is_key_pressed[usize::from(self.v[x])] {
                            self.pc += 2;
                        }
                    }
                    0x00A1 => {
                        if !self.is_key_pressed[usize::from(self.v[x])] {
                            self.pc += 2;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            0xF000 => {
                let x = usize::from((instruction & 0x0F00) >> 8);

                match instruction & 0x00FF {
                    0x0007 => {
                        self.v[x] = self.delay_timer;
                    }
                    0x000A => {
                        if let Some(key) = self.is_key_pressed.iter().position(|&pressed| pressed) {
                            self.v[x] = key as u8;
                        } else {
                            self.pc -= 2;
                        }
                    }
                    0x0015 => {
                        self.delay_timer = self.v[x];
                    }
                    0x0018 => {
                        self.sound_timer = self.v[x];
                    }
                    0x001E => {
                        self.i += u16::from(self.v[x]);
                    }
                    0x0029 => {
                        self.i = u16::from(self.v[x] & 0x0F) * SIZE_OF_SPRITE_FOR_DIGIT;
                    }
                    0x0033 => {
                        bus.ram_write_byte(self.i, self.v[x] / 100);
                        bus.ram_write_byte(self.i + 1, (self.v[x] / 10) % 10);
                        bus.ram_write_byte(self.i + 2, self.v[x] % 10);
                    }
                    0x0055 => {
                        for offset in 0..=x {
                            bus.ram_write_byte(self.i + offset as u16, self.v[offset]);
                        }
                        self.i += x as u16 + 1;
                    }
                    0x0065 => {
                        for offset in 0..=x {
                            self.v[offset] = bus.ram_read_byte(self.i + offset as u16);
                        }
                        self.i += x as u16 + 1;
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
