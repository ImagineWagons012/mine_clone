use mine_clone::state::State;
use tokio::{runtime::Builder, sync::mpsc};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Error)
        .init();

    let runtime = Builder::new_multi_thread().worker_threads(8).enable_all().build().unwrap();

    let (event_tx, mut event_rx) = mpsc::unbounded_channel();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    window.set_cursor_visible(false);

    runtime.block_on(async {
        let mut state = State::new(window).await;
        state.configure();

        runtime.spawn(async move {
            'main_loop: loop {
                let event = event_rx.recv().await.unwrap();
                match &event {
                    Event::LoopExiting => {
                        break 'main_loop;
                    }
                    Event::DeviceEvent { event, .. } => match event {
                        DeviceEvent::MouseMotion { delta } => {
                            state.camera_controller.process_mouse(delta.0, delta.1);
                        }
                        _ => {}
                    },
                    Event::WindowEvent { event, .. } => {
                        if !state.input(event) {
                            match event {
                                WindowEvent::RedrawRequested => {
                                    state.window.request_redraw();
                                    state.update().await;
                                    match state.render() {
                                        Ok(_) => (),
                                        Err(wgpu::SurfaceError::OutOfMemory) => {
                                            log::error!("Out of memory")
                                        }
                                        Err(wgpu::SurfaceError::Timeout) => {
                                            log::warn!("Surface Timeout")
                                        }
                                        Err(
                                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                        ) => state.resize(state.size),
                                    }
                                }
                                WindowEvent::Resized(size) => {
                                    state.resize(*size);
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        });
    });

    let _ = event_loop.run(move |event, control_flow| {
        match &event {
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => control_flow.exit(),
            // Event::WindowEvent { window_id: _, event: WindowEvent::RedrawRequested } => {
            //     window.request_redraw();
            // } 
            _ => (),
        }
        event_tx.send(event).unwrap();
    });
}
