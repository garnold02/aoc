use std::str::Lines;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut cpu = Cpu::new();
    let mut i = 0;
    let mut sum = 0;
    let mut check = false;

    let mut crt = Crt::new(40, 6);

    loop {
        i += 1;

        if check && (i - 20) % 40 == 0 {
            sum += i * cpu.x;
        }

        if i == 20 {
            sum += i * cpu.x;
            check = true;
        }

        let lit = ((crt.pos % 40) - cpu.x).abs() <= 1;
        crt.draw(lit);

        cpu.cycle();

        if cpu.done {
            break;
        }
    }

    println!("{sum}");

    for y in 0..6 {
        for x in 0..40 {
            if crt.buf[y * 40 + x] {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!()
    }
}

struct Cpu {
    x: i32,
    addx_state: AddxState,
    instructions: Lines<'static>,
    done: bool,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            addx_state: AddxState::Idle,
            instructions: INPUT.lines(),
            done: false,
        }
    }

    fn cycle(&mut self) {
        match self.addx_state {
            AddxState::Idle => {
                if let Some(line) = self.instructions.next() {
                    let mut split = line.split(' ');
                    let opcode = split.next().unwrap();

                    match opcode {
                        "noop" => (),

                        "addx" => {
                            let value = split.next().unwrap().parse().unwrap();
                            self.addx_state = AddxState::Wait(value);
                        }

                        _ => panic!(),
                    }
                } else {
                    self.done = true;
                }
            }

            AddxState::Wait(value) => {
                self.x += value;
                self.addx_state = AddxState::Idle;
            }
        }
    }
}

enum AddxState {
    Idle,
    Wait(i32),
}

struct Crt {
    buf: Vec<bool>,
    pos: i32,
}

impl Crt {
    fn new(width: usize, height: usize) -> Self {
        let buf = vec![false; width * height];
        Self { buf, pos: 0 }
    }

    fn draw(&mut self, lit: bool) {
        if let Some(pixel) = self.buf.get_mut(self.pos as usize) {
            *pixel = lit;
            self.pos += 1;
        }
    }
}
