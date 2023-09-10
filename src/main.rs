mod ui;
mod util;
mod synth;

use std::{rc::Rc, sync::Mutex};
use hecs::World;
use pixels::{Pixels, SurfaceTexture};
use wasm_bindgen::JsCast;
use web_sys::console;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
    dpi::LogicalSize,
};

use crate::{synth::{Vco, VcoWaveform}, ui::Frame, util::{Pos, Size, Parent, Transform}};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const PADDING: u32 = 30;

const RESIZE_DEBOUNCE_MS: i32 = 100;
static RESIZE_TIMEOUT_ID: Mutex<i32> = Mutex::new(0);

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_bindgen_futures::spawn_local(run());
}

async fn run() {
    use winit::platform::web::EventLoopExtWebSys;

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("cerise")
        .build(&event_loop)
        .unwrap();
    let window = Rc::new(window);

    insert_canvas(Rc::clone(&window));
    
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(WIDTH, HEIGHT, surface_texture).await.unwrap()
    }; 
    
    let mut world = World::new();
    
    let frame = world.spawn((
        Frame {
            title: "oscillator".into()
        },
        Pos(50, 50),
        Size(260, 60)
    ));

    let vco = world.spawn((
        Vco {
            waveform: VcoWaveform::Sine,
            frequency: 440.0,
            amplitude: 0.5
        }, 
        Size(250, 50),
        Parent {
            entity: frame,
            transform: Transform(10, 10)
        }
    ));
    
    event_loop.spawn(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent { 
                event: WindowEvent::Resized(size),
                ..
            } => {
                pixels.resize_surface(size.width, size.height).unwrap();
            },
            Event::RedrawRequested(_) => {
                redraw_world(&mut world, pixels.frame_mut());
                pixels.render().unwrap();

                /*
                for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as u32;
                    let y = (i / WIDTH as usize) as u32;

                    let rgba = if x >= 270 && y >= 190 && x < 370 && y < 290 {
                        [0xe3, 0x7b, 0x8f, 0xff]
                    } else {
                        [0x15, 0x1e, 0x24, 0xff]
                    };
                    
                    pixel.copy_from_slice(&rgba);
                }
                pixels.render().unwrap();
                */
            },
            _ => ()
        };
        
        update_world(&world);
        window.request_redraw();
    });
}

fn redraw_world(world: &mut World, frame: &mut [u8]) {
    let mut lines = Vec::<(Pos, Pos)>::new();

    // draw frames
    for (_id, (_frame, pos, size)) in world.query_mut::<(&Frame, &Pos, &Size)>() {
        lines.push((Pos(pos.0, pos.1), Pos(pos.0 + size.0, pos.1)));
        lines.push((Pos(pos.0, pos.1), Pos(pos.0, pos.1 + size.1)));
        lines.push((Pos(pos.0 + size.0, pos.1), Pos(pos.0 + size.0, pos.1 + size.1)));
        lines.push((Pos(pos.0, pos.1 + size.1), Pos(pos.0 + size.0, pos.1 + size.1)));
    }
    
    console::log_1(&serde_json::to_string(&lines).unwrap().into());
    
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as i32;
        let y = (i / WIDTH as usize) as i32;
        
        let rgba = if (&*lines).into_iter().any(
            |line| x >= line.0.0 && y >= line.0.1 &&
                   x <= line.1.0 && y <= line.1.1) {
            [0xe3, 0x7b, 0x8f, 0xff]
                   } else {
            [0x15, 0x1e, 0x24, 0xff]
                   };
        
        pixel.copy_from_slice(&rgba);
    }
}

fn update_world(world: &World) {
    // 
}

fn insert_canvas(window: Rc<Window>) {
    use winit::platform::web::WindowExtWebSys;
    
    let canvas = window.canvas();
    canvas.style().set_property("margin", "5px").unwrap();
    
    let get_window_size = || {
        let client_window = web_sys::window().unwrap();
        LogicalSize::new(
            client_window.inner_width().unwrap().as_f64().unwrap() - PADDING as f64,
            client_window.inner_height().unwrap().as_f64().unwrap() - PADDING as f64,
        )
    };
    
    window.set_inner_size(get_window_size());
    
    let client_window = web_sys::window().unwrap();
    
    client_window.document()
        .and_then(|document| document.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(canvas)).ok()
        })
        .unwrap();

    // create a closure to resize winit window when browser is resized
    let resize = wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |_e: web_sys::Event| {
        let size = get_window_size();
        window.set_inner_size(size)
    });
    let closure = wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |_e: web_sys::Event| {
        let client_window = web_sys::window().unwrap();
        let mut handle = RESIZE_TIMEOUT_ID.lock().unwrap();
        client_window.clear_timeout_with_handle(*handle);
        *handle = client_window.set_timeout_with_callback_and_timeout_and_arguments_0(
            resize.as_ref().unchecked_ref(),
            RESIZE_DEBOUNCE_MS
        ).unwrap();
    });
    client_window
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}