extern crate sdl2;

#[allow(non_snake_case)]
#[allow(unused_variables)]
mod bitty;
mod gameboy;
mod graphics;
mod debugger;


use debugger::Debugger;
use gameboy::Gameboy;
use gameboy::rom::Rom;

#[macro_use]
extern crate log;
extern crate log4rs;

use log::LogLevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use graphics::display::Display;
use std::env;

fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LogLevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();

    let context = ::sdl2::init().unwrap();
    let event_pump = context.event_pump();
    let mut display = Display::new(context);
    let filename = env::args().nth(1).unwrap();
    println!("{}", filename);
    let filepath = format!("/Users/patallen/Code/Emulators/GameRoy/{}", filename);;
    let rom = Rom::new(&*filepath);
    let mut gameboy = Gameboy::new(rom);
    gameboy.mmu.ppu.set_on_refresh(Box::new(move | arr | { display.draw_frame(arr); }));
    let mut debugger = Debugger::new(gameboy, event_pump.unwrap());
	debugger.run();
}
