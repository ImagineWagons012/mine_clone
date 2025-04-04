use mine_clone::state::State;
use winit::{
    event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::WindowBuilder
};

#[tokio::main]
async fn main() {
    env_logger::builder()
    .target(env_logger::Target::Stdout)
    .format_timestamp(None)
    .filter_level(log::LevelFilter::Info)
    .init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;
    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(window.current_monitor().unwrap()))));
    window.set_cursor_visible(false);
    state.configure();

    let _ = event_loop.run(move |event, control_flow| match event {
        Event::DeviceEvent { event, .. } => {
            match event {
                DeviceEvent::MouseMotion { delta } => {
                    state.camera_controller.process_mouse(delta.0, delta.1);
                }
                _ => {}
            }
        },
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        pollster::block_on(state.resize(*physical_size));
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::RedrawRequested => {
                        state.time.set_frame_start_time();
                        // This tells winit that we want another frame after this one
                        log::info!("request redraw");
                        state.window().request_redraw();

                        pollster::block_on(state.update());
                        match pollster::block_on(state.render()) {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                pollster::block_on(state.resize(state.size))
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                log::error!("OutOfMemory");
                                control_flow.exit();
                            }

                            // This happens when the a frame takes too long to present
                            Err(wgpu::SurfaceError::Timeout) => {
                                log::warn!("Surface timeout")
                            }
                        }
                        state.time.update_frame_time();
                        log::info!(
                            "frame time: {}, fps: {}, CPU time: {}, GPU time: {}", 
                            state.time.delta_time() * 1000.0, 1.0 / state.time.delta_time(), 
                            state.time.cpu_time() * 1000.0, state.time.gpu_time() * 1000.0
                        );
                    }
                    _ => {}
                }
            }
        },
        
        _ => {}
    });
}
