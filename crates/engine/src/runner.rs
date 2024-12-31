use std::{cell::RefCell, rc::Rc};

use bevy::app::{App, AppExit, Plugin, PluginsState};
use bevy::utils::{Duration, Instant};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{js_sys, WorkerGlobalScope};

pub struct TimeoutRunnerPlugin {
    duration: Duration,
}

impl TimeoutRunnerPlugin {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl Plugin for TimeoutRunnerPlugin {
    fn build(&self, app: &mut App) {
        let duration = Some(self.duration);

        app.set_runner(move |mut app| {
            let plugins_state = app.plugins_state();
            if plugins_state != PluginsState::Cleaned {
                while app.plugins_state() == PluginsState::Adding {}
                app.finish();
                app.cleanup();
            }

            let mut tick = move |wait: Option<Duration>| -> Result<Option<Duration>, AppExit> {
                let start_time = Instant::now();

                app.update();

                if let Some(exit) = app.should_exit() {
                    return Err(exit);
                };

                let end_time = Instant::now();

                if let Some(wait) = wait {
                    let exe_time = end_time - start_time;
                    if exe_time < wait {
                        return Ok(Some(wait - exe_time));
                    }
                }

                Ok(None)
            };

            let asap = Duration::from_millis(1);

            let exit = Rc::new(RefCell::new(AppExit::Success));
            let closure = Rc::new(RefCell::new(None));

            let tick_app = {
                let exit = exit.clone();
                let closure = closure.clone();
                move || {
                    match tick(duration) {
                        Ok(delay) => {
                            set_timeout(closure.borrow().as_ref().unwrap(), delay.unwrap_or(asap));
                        }
                        Err(code) => {
                            exit.replace(code);
                        }
                    };
                }
            };

            *closure.borrow_mut() = Some(Closure::wrap(Box::new(tick_app) as Box<dyn FnMut()>));
            set_timeout(closure.borrow().as_ref().unwrap(), asap);

            exit.take()
        });
    }
}

fn set_timeout(callback: &Closure<dyn FnMut()>, duration: Duration) {
    js_sys::global()
        .dyn_into::<WorkerGlobalScope>()
        .expect("Should return WorkerGlobalScope.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            duration.as_millis() as i32,
        )
        .expect("Should register `setTimeout`.");
}
