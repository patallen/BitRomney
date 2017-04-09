mod command;

use std::process;
use std::io::{stdout, stdin, Write};

use gameboy::Gameboy;
use operations::{Operation, get_operation};
use self::command::{Command, build_step, build_show, build_set, ShowType, SetType};


const MEM_DISPLAY_WIDTH: u16 = 16;


enum DebugMode {
    Quitting,
    Running,
    Restarting,
    Stepping,
    Repl,
}

pub struct Debugger {
    tracepoints: Vec<u16>,
    breakpoints: Vec<usize>,
    mode: DebugMode,
    gameboy: Gameboy,
    step_distance: u16,
}

impl Debugger {
    pub fn new(rompath: &str) -> Debugger {
        Debugger {
            tracepoints: Vec::new(),
            breakpoints: Vec::new(),
            gameboy: Gameboy::new(rompath),
            mode: DebugMode::Repl,
            step_distance: 10,
        }
    }
    fn cycle(&mut self) {
        self.gameboy.step();
        self.print();
        self.check_breakpoints();
    }
    fn print(&mut self) {
        let op = self.next_operation();
        println!("(PC:{:04X}|SP:{:04X}) :: {:?}",
                 self.gameboy.cpu.regs.pc, self.gameboy.cpu.regs.sp, op);
    }
    fn check_breakpoints(&mut self) {
        let pc = self.gameboy.cpu.regs.pc;
        if self.breakpoints.iter().any(|x| *x == pc) {
            self.mode = DebugMode::Repl;
        }
    }
    fn step(&mut self) {
        for _ in 0..self.step_distance {
            self.cycle()
        }
        self.mode = DebugMode::Repl;
    }
    fn next_operation(&mut self) -> Operation {
        let first = self.gameboy.mmu.read(self.gameboy.cpu.regs.pc) as u16;
        let code = match self.gameboy.mmu.read(self.gameboy.cpu.regs.pc) {
            0xCB => { first << 8 | self.gameboy.mmu.read(self.gameboy.cpu.regs.pc + 1) as u16 },
            _ => first
        };
        get_operation(code)
    }
    pub fn run(&mut self) {
        loop {
            match self.mode {
                DebugMode::Repl => self.repl(),
                DebugMode::Restarting => {},
                DebugMode::Quitting => process::exit(1),
                DebugMode::Running => self.cycle(),
                DebugMode::Stepping => self.step(),
            };
        }
    }
    fn repl(&mut self) {
        /// The Repl is a single function that allows the user to
        /// change the state of the debugger and or emulator using a set
        /// of comprehensive commands and arguments.
        loop {
            print!("gbdb> ");
            stdout().flush().unwrap();

            match parse_input(&read_stdin()) {
                Ok(command) => {self.handle_command(command); break;}
                Err(error) => {println!("{}", error)}
            };
        }
    }
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Quit => self.mode = DebugMode::Quitting,
            Command::Restart => self.mode = DebugMode::Restarting,
            Command::Resume => self.mode = DebugMode::Running,
            Command::Step(dist) => {self.step_distance = dist; self.mode = DebugMode::Stepping },
            Command::Show(showtype) => self.show(showtype),
            Command::Set(settype) => self.set(settype),
            _ => {}
        }
    }
    fn set(&mut self, settype: SetType) {
        match settype {
            SetType::Breakpoint(val) => { self.breakpoints.push(val)},
            _ => println!("Set type not currenty supported.")
        }
    }
    fn show(&self, showtype: ShowType) {
        match showtype {
            ShowType::Registers => self.print_registers(),
            ShowType::Tracepoints => self.print_tracepoints(),
            ShowType::Memory(low, hi) => self.print_memory(low, hi),
            ShowType::Breakpoints => self.print_breakpoints(),
        }
    }
    fn print_registers(&self) {
        let regs = &self.gameboy.cpu.regs;
        println!("");
        println!("-----8-bit Registers-----");
        println!("B: {:02X} | C: {:02X} || BC: {:04X}", regs.b, regs.c, regs.bc());
        println!("D: {:02X} | E: {:02X} || DE: {:04X}", regs.d, regs.e, regs.de());
        println!("H: {:02X} | L: {:02X} || HL: {:04X}", regs.h, regs.l, regs.hl());
        println!("A: {:02X} | F: {:02X} || AF: {:04X}", regs.a, regs.flags.as_u8(), regs.af());
        println!("----Address Registers----");
        println!("  PC: {:04X} | SP: {:04X}", regs.pc, regs.sp);
        println!("----------Flags----------");
        println!("{:?}", self.gameboy.cpu.regs.flags);
        println!("")
    }
    fn print_tracepoints(&self) {
    }
    fn print_breakpoints(&self) {
    }
    fn print_memory(&self, low: u16, hi: u16) {
        let mem_width = MEM_DISPLAY_WIDTH as usize;
        let l = low as usize / mem_width * mem_width;
        let h = hi as usize / mem_width * mem_width + mem_width;

        let mems = self.gameboy.mmu.read_range(l, h);

        let mut lines: Vec<String> = Vec::new();
        for (i, ch) in mems.as_slice().chunks(mem_width).enumerate() {
            let string = ch.into_iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join(" ");
            let line = format!("0x{:04X} | {}", i * mem_width + l as usize, string);
            lines.push(line);
        }
        let header = (0..mem_width).into_iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join(" ");
        println!("       | {}", header);
        println!("--------------------------------------------------------");
        println!("{}", lines.join("\n"));
    }
}


fn parse_input(text: &str) -> Result<Command, &str> {
    let parts: Vec<&str> = text.split(" ").collect();
    let next_parts = &parts[1..].to_vec();
    match parts[0] {
        "restart" | "r"           => Ok(Command::Restart),
        "go" | "resume" | "start" => Ok(Command::Resume),
        "exit" | "quit" | "q"     => Ok(Command::Quit),
        "show" | "print"          => build_show(next_parts),
        "step"                    => build_step(next_parts),
        "set"                     => build_set(next_parts),
        _                         => return Err("Invalid command.")
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}

fn str_to_u16(string: &str) -> Result<u16, &str> {
    let mut string = string;

    if string.starts_with("0x") {
        string = &string[2..];
        match u16::from_str_radix(string, 16) {
            Ok(res) => return Ok(res),
            _ => return Err("Could not convert to u16 from hex string.")
        }
    } else {
        match str::parse::<u16>(string) {
            Ok(res) => return Ok(res),
            _ => return Err("Could not convert to u16 from string.")
        }
    }
}
