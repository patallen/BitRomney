use minifb::{Window, WindowOptions};
const DISPLAY_WIDTH_PIXELS: usize = 160;
const DISPLAY_HEIGHT_PIXELS: usize = 144;
const SCALE: u8 = 2;
const TITLE: &'static str = "BitRomney GB";

pub struct Dims {
    pub width: usize,
    pub height: usize,
}

impl Default for Dims {
    fn default() -> Dims {
        Self {
            width: DISPLAY_WIDTH_PIXELS,
            height: DISPLAY_HEIGHT_PIXELS,
        }
    }
}

#[derive(Debug)]
pub struct KeyState {
    q: bool,
    p: bool,
    h: bool,
    space: bool,
}

impl KeyState {
    pub fn default() -> KeyState {
        Self {
            q: false,
            p: false,
            h: false,
            space: false,
        }
    }
}

pub struct Display {
    window: minifb::Window,
    dims: Dims,
    scale: u8,
    keystate: KeyState,
}

#[derive(Debug)]
pub enum Key {
    Q,
    H,
    P,
    Space,
}

#[derive(Debug)]
pub enum EventKind {
    KeyDown(Key),
    KeyUp(Key),
}

pub struct Event {
    kind: EventKind,
}

impl Display {
    pub fn new() -> Display {
        let dims = Dims::default();
        let window = Window::new(
            TITLE,
            dims.width,
            dims.height,
            WindowOptions {
                scale: minifb::Scale::X2,
                ..minifb::WindowOptions::default()
            },
        )
        .expect("Failed to create window.");

        Display {
            window,
            dims,
            scale: SCALE,
            keystate: KeyState::default(),
        }
    }

    pub fn draw_frame(&mut self, data: &[u32]) {
        self.window.update_with_buffer(&data).unwrap();
    }

    #[allow(dead_code)]
    pub fn get_events(&self) -> Vec<Event> {
        let mut events = vec![];
        if self.window.is_key_down(minifb::Key::H) != self.keystate.h {
            let kind = match self.keystate.h {
                true => EventKind::KeyDown(Key::H),
                false => EventKind::KeyUp(Key::H),
            };
            events.push(Event { kind });
        }

        if self.window.is_key_down(minifb::Key::Space) != self.keystate.space {
            let kind = match self.keystate.space {
                true => EventKind::KeyDown(Key::Space),
                false => EventKind::KeyUp(Key::Space),
            };
            events.push(Event { kind });
        }

        if self.window.is_key_down(minifb::Key::P) != self.keystate.p {
            let kind = match self.keystate.p {
                true => EventKind::KeyDown(Key::P),
                false => EventKind::KeyUp(Key::P),
            };
            events.push(Event { kind });
        }

        if self.window.is_key_down(minifb::Key::Q) != self.keystate.q {
            let kind = match self.keystate.q {
                true => EventKind::KeyDown(Key::Q),
                false => EventKind::KeyUp(Key::Q),
            };
            events.push(Event { kind });
        }
        events
    }
}
