use crate::debugger::str_to_u16;

#[derive(Debug)]
pub enum ShowType {
    Memory(u16, u16),
    // Breakpoints,
    Tracepoints,
    Registers,
}

#[derive(Debug)]
pub enum SetType {
    // Tracepoint,
    Memory(usize, u8),
    Breakpoint(usize),
    // Register,
    // Speed,
}

#[derive(Debug)]
pub enum Command {
    Show(ShowType),
    Set(SetType),
    Step(u32),
    Restart,
    Resume,
    Quit,
    Help,
}

pub fn build_step(parts: &Vec<&str>) -> Result<Command, &'static str> {
    match parts[0].parse::<u32>() {
        Ok(val) => Ok(Command::Step(val)),
        Err(_) => Err("Invalid arguments for 'step'."),
    }
}

pub fn build_show(parts: &Vec<&str>) -> Result<Command, &'static str> {
    let st = parts[0];
    let showtype = match st {
        "regs" | "registers" => ShowType::Registers,
        "tracepoints" | "tps" | "traces" => ShowType::Tracepoints,
        "mem" | "memory" => match _build_memory_type(&parts[1..].to_vec()) {
            Ok(memtype) => memtype,
            Err(err) => return Err(err),
        },
        _ => return Err("That is not a valid 'show' type."),
    };
    Ok(Command::Show(showtype))
}
pub fn build_set(parts: &Vec<&str>) -> Result<Command, &'static str> {
    let st = parts[0];
    let settype = match st {
        "bp" | "breakpoint" | "break" => match _build_breakpoint(&parts) {
            Ok(settype) => settype,
            Err(err) => return Err(err),
        },
        "mem" | "memory" | "m" => match _build_memory_set(&parts) {
            Ok(settype) => settype,
            Err(err) => return Err(err),
        },
        _ => return Err("Invalid argument for 'set'."),
    };
    Ok(Command::Set(settype))
}

pub fn _build_memory_set(parts: &Vec<&str>) -> Result<SetType, &'static str> {
    if parts.len() < 3 {
        return Err("Set mem requires two arguments.");
    }
    let arg1 = match str_to_u16(parts[1]) {
        Ok(val) => val as usize,
        Err(_) => return Err("Invalid argument 1 for memory set."),
    };
    let arg2 = match str_to_u16(parts[2]) {
        Ok(val) => val as u8,
        Err(_) => return Err("Invalid argument 1 for memory set."),
    };

    Ok(SetType::Memory(arg1, arg2))
}

fn _build_breakpoint(parts: &Vec<&str>) -> Result<SetType, &'static str> {
    if parts.len() < 2 {
        return Err("Breakpoint requires an address argument.");
    }
    match str_to_u16(parts[1]) {
        Ok(val) => Ok(SetType::Breakpoint(val as usize)),
        Err(_) => return Err("Invalid argument for set type."),
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
