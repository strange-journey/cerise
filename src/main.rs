use std::rc::Rc;
use pixels::{Pixels, SurfaceTexture};
use wasm_bindgen::JsCast;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
    dpi::LogicalSize,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const PADDING: u32 = 30;

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
                for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as i16;
                    let y = (i / WIDTH as usize) as i16;

                    let rgba = if x >= 270 && y >= 190 && x < 370 && y < 290 {
                        [0xe3, 0x7b, 0x8f, 0xff]
                    } else {
                        [0x15, 0x1e, 0x24, 0xff]
                    };
                    
                    pixel.copy_from_slice(&rgba);
                }
                pixels.render().unwrap();
            },
            _ => ()
        };
        
        window.request_redraw();
    });
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
    let closure = wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |_e: web_sys::Event| {
        let size = get_window_size();
        window.set_inner_size(size)
    });
    client_window
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}