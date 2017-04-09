use gameboy::Gameboy;
use operations::{Operation, get_operation};

use std::io::{Write, stdout, stdin};
use std::process;

#[derive(Debug)]
enum ShowType {
    Memory(u16, u16),
    Breakpoints,
    Tracepoints,
    Registers,
}

#[derive(Debug)]
enum SetType {
    Tracepoint,
    Breakpoint(u16),
    Register,
    Speed,
}
#[derive(Debug)]
enum Command {
    Show(ShowType),
    Set(SetType),
    Step(u16),
    Restart,
    Resume,
    Quit,
    Help,
}
enum DebugMode {
    Quitting,
    Running,
    Restarting,
    Stepping,
    Repl,
}
pub struct Debugger {
    tracepoints: Vec<u16>,
    breakpoints: Vec<u16>,
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
    pub fn start(&mut self) {
        self.run();
    }
    fn restart(&mut self) {
        self.reset();
        self.mode = DebugMode::Repl;
    }
    fn reset(&mut self) {
    }
    fn cycle(&mut self) {
        self.gameboy.step();
        let pc = self.gameboy.cpu.regs.pc;
        let sp  = self.gameboy.cpu.regs.sp;
        if self.breakpoints.iter().any(|x| *x == pc as u16) {
            self.mode = DebugMode::Repl;
        }
        let op = self.next_operation();
        println!("(PC:{:04X}|SP:{:04X}) :: {:?}", pc, sp, op);
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
    fn run(&mut self) {
        loop {
            match self.mode {
                DebugMode::Repl => self.repl(),
                DebugMode::Restarting => self.restart(),
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
            let input = read_stdin();
            let result = parse_input(&input);

            match result {
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
        let l = low / 16 * 16;
        let h = hi / 16 * 16 + 16;

        let mut mems: Vec<u8> = Vec::new();
        for addr in l..h {
            mems.push(self.gameboy.mmu.read(addr as usize));
        }
        print!("\n0x**** | ");
        for x in 0.. 16 {
            print!("{:02X} ", x);
        }
        println!("\n--------------------------------------------------------");
        for (i, ch) in mems.as_slice().chunks(16).enumerate() {
            print!("0x{:04X} | ", low + (i * 16) as u16);
            for x in ch {
                print!("{:02X} ", x);
            }
            print!("\n")
        }
    }
}


fn parse_input(text: &str) -> Result<Command, &str> {
    let parts: Vec<&str> = text.split(" ").collect();
    let cmd = parts[0];
    let next_parts = &parts[1..].to_vec();
    match cmd {
        "restart" | "r"           => Ok(Command::Restart),
        "go" | "resume" | "start" => Ok(Command::Resume),
        "exit" | "quit" | "q"     => Ok(Command::Quit),
        "show" | "print"          => build_show(next_parts),
        "step"                    => build_step(next_parts),
        "set"                     => build_set(next_parts),
        _                         => return Err("Invalid command.")
    }
}

fn build_step(parts: &Vec<&str>) -> Result<Command, &'static str> {
    match parts[0].parse::<u16>() {
        Ok(val) => Ok(Command::Step(val)),
        Err(_) => Err("Invalid arguments for 'step'."),
    }
}

fn build_show(parts: &Vec<&str>) -> Result<Command, &'static str> {
    let st = parts[0];
    let showtype = match st {
        "regs" | "registers" => ShowType::Registers,
        "tracepoints" | "tps" | "traces" => ShowType::Tracepoints,
        "mem" | "memory" => match _build_memory_type(&parts[1..].to_vec()) {
            Ok(memtype) => memtype,
            Err(err) => return Err(err)
        },
        _ => return Err("That is not a valid 'show' type."),
    };
    Ok(Command::Show(showtype))
}
fn build_set(parts: &Vec<&str>) -> Result<Command, &'static str> {
    let st = parts[0];
    let settype = match st {
        "bp" | "breakpoint" | "break" => match _build_breakpoint(&parts) {
            Ok(settype) => settype,
            Err(err) => return Err(err)
        },
        _ => return Err("Invalid argument for 'set'.")

    };
    Ok(Command::Set(settype))
}

fn _build_breakpoint(parts: &Vec<&str>) -> Result<SetType, &'static str> {
    if parts.len() < 2 {
        return Err("Breakpoint requires an address argument.")
    }
    match str_to_u16(parts[1]) {
        Ok(val) => Ok(SetType::Breakpoint(val)),
        Err(_) => return Err("Invalid argument for set type.")
    }
}
fn _build_memory_type(parts: &Vec<&str>) -> Result<ShowType, &'static str> {
    let loc1 = match str_to_u16(parts[0]) {
        Ok(val) => val,
        Err(_) => return Err("Invalid argument for memory type."),
    };

   let mut loc2: u16 = loc1;
    if parts.len() > 1 {
        loc2 = match str_to_u16(parts[1]) {
            Ok(val) => val,
            Err(_) => return Err("Invalid argument for memory type."),
        };
    }
    Ok(ShowType::Memory(loc1, loc2))
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
