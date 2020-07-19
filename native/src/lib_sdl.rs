#[macro_use]
extern crate neon;
extern crate sdl2;

use neon::context::Context;
use neon::types::JsUndefined;
use std::time::Duration;

pub struct Canvas();

declare_types! {
    pub class JsCanvas for Canvas {
        init(mut _cx) {
            Ok(Canvas())
        }

        method open(mut cx) {
            let sdl_context = sdl2::init()
            .or_else(|err| cx.throw_error(err.to_string()))?;

            let video_subsystem = sdl_context.video()
                .or_else(|err| cx.throw_error(err.to_string()))?;

            let _window = video_subsystem
                .window("Game", 900, 700)
                .resizable()
                .build()
                .or_else(|err| cx.throw_error(err.to_string()))?;

            let mut event_pump = sdl_context.event_pump().or_else(|err| cx.throw_error(err.to_string()))?;
            'main: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'main,
                        _ => {}
                    }
                }
                // render window contents here
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
            Ok(JsUndefined::new().upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsCanvas>("Canvas")?;

    Ok(())
});
