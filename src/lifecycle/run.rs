use crate::{
    Result,
    backend::Backend,
    geom::Vector,
    input::{ButtonState, MouseButton, KEY_LIST, LINES_TO_PIXELS},
    lifecycle::{Application, Event, State, Settings, Window},
};
use {
    std::env::set_current_dir,
    winit::{
        event::{ElementState, Event as WinitEvent, MouseScrollDelta, WindowEvent},
        event_loop::ControlFlow
    }
};

/// Run the application's game loop
///
/// On desktop platforms, this yields control to a simple game loop controlled by a Timer. On wasm,
/// this yields control to the browser functions setInterval and requestAnimationFrame
pub fn run<T: State>(title: &str, size: Vector, settings: Settings) {
    run_with(title, size, settings, || T::new());
}

/// Run the application's game loop
///
/// On desktop platforms, this yields control to a simple game loop controlled by a Timer. On wasm,
/// this yields control to the browser functions setInterval and requestAnimationFrame
///
/// This function behaves the same as `run`, but the `FnOnce` argument is responsible for state
/// creation instead of the backend.
pub fn run_with<T: State, F: FnOnce()->Result<T>>(title: &str, size: Vector, settings: Settings, f: F) {
    if let Err(error) = run_impl::<T, F>(title, size.into(), settings, f) {
        T::handle_error(error);
    }
}

fn run_impl<T: State, F: FnOnce()->Result<T>>(title: &str, size: Vector, settings: Settings, f: F) -> Result<()> {
    // A workaround for https://github.com/koute/cargo-web/issues/112
    if let Err(_) = set_current_dir("static") {
        eprintln!("Warning: no asset directory found. Please place all your assets inside a directory called 'static' so they can be loaded");
        eprintln!("Execution continuing, but any asset-not-found errors are likely due to the lack of a 'static' directory.")
    }
    let (window, event_loop) = Window::build(title, size, settings)?;
    #[cfg(feature = "sounds")]
    crate::sound::Sound::initialize();
    let mut app: Application<T> = Application::new(window, f)?;

    event_loop.run(move |event, _, mut ctrl| {
        match event {
            WinitEvent::NewEvents(winit::event::StartCause::Init) => {
                app.window.backend().window().request_redraw();
            }
            WinitEvent::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                let size: Vector = size.into();
                // Glutin reports a resize to 0, 0 when minimizing the window
                if size.x != 0.0 && size.y != 0.0 {
                    app.window.adjust_size(size);
                }
            }
            WinitEvent::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                if let Err(error) = tick(&mut app) {
                    T::handle_error(error);
                }
                app.window.backend().window().request_redraw();
            }
            WinitEvent::WindowEvent { event, .. } => {
                if let Some(evt) = window_event(&app.window, event, &mut ctrl) {
                    app.event_buffer.push(evt);
                }
            }
            _ => ()
        }

        if !app.window.is_running() {
            *ctrl = ControlFlow::Exit;
        }
    })
}

fn tick<T: State>(app: &mut Application<T>) -> Result<()> {
    app.update()?;
    app.draw()?;

    Ok(())
}

fn window_event(window: &Window, evt: WindowEvent, ctrl: &mut ControlFlow) -> Option<Event> {
    match evt {
        WindowEvent::CloseRequested => {
            *ctrl = ControlFlow::Exit;

            Some(Event::Closed)
        }
        WindowEvent::KeyboardInput { input: event, .. } => {
            if let Some(keycode) = event.virtual_keycode {
                let state = match event.state {
                    ElementState::Pressed => ButtonState::Pressed,
                    ElementState::Released => ButtonState::Released
                };
                let key = KEY_LIST[keycode as usize];

                Some(Event::Key(key, state))
            } else {
                None
            }
        }
        WindowEvent::ReceivedCharacter(character) if character.is_alphanumeric() => {
            Some(Event::Typed(character))
        }
        WindowEvent::CursorMoved { position, .. } => {
            let position: Vector = position.into();
            let position = window.project() * (position - window.screen_offset());
            Some(Event::MouseMoved(position))
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let value = match state {
                ElementState::Pressed => ButtonState::Pressed,
                ElementState::Released => ButtonState::Released,
            };
            let index = match button {
                winit::event::MouseButton::Left => MouseButton::Left,
                winit::event::MouseButton::Right => MouseButton::Right,
                winit::event::MouseButton::Middle => MouseButton::Middle,
                // Other mouse buttons just mean we should move on to the next winit event
                _ => { return None; },
            };
            Some(Event::MouseButton(index, value))
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let vector = match delta {
                MouseScrollDelta::LineDelta(x, y) => Vector::new(x, -y) * LINES_TO_PIXELS,
                MouseScrollDelta::PixelDelta(delta) => delta.into()
            };
            Some(Event::MouseWheel(vector))
        }
        _ => None
    }
}
