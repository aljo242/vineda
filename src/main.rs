extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsytem = sdl.video().unwrap();
    let window = video_subsytem
        .window("vineda", 900, 700)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{ .. } => break 'main,
             _ => {}
            }
        }
        // do rendering here
    }
}
