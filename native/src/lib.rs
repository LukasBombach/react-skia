#[macro_use]
extern crate neon;

// use std::thread;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// use neon::types::JsUndefined;

pub struct Canvas();

declare_types! {
    pub class JsCanvas for Canvas {
        init(mut _cx) {
            Ok(Canvas())
        }

        method open(mut _cx) {
            //thread::spawn(move || {
            //println!("rust: spawned a thread");

            let event_loop = EventLoop::new();
            let window = WindowBuilder::new().build(&event_loop).unwrap();
            println!("rust: post window");

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        println!("The close button was pressed; stopping");
                        *control_flow = ControlFlow::Exit
                    },
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    },
                    Event::RedrawRequested(_) => {
                    },
                    _ => ()
                }
            });

            //});
            // // some work here
            // let _res = child.join();
            //Ok(JsUndefined::new().upcast())

        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsCanvas>("Canvas")?;

    Ok(())
});
