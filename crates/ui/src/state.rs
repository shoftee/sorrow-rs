use std::rc::Rc;

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use sorrow_core::{
    communication::{Intent, Notification},
    state::{calendar::SeasonKind, precision::Precision, resources::Kind, time::RunningState},
};
use sorrow_engine::Endpoint;

pub struct OptionSignals {
    pub precision: RwSignal<Precision>,
}

pub struct ResourceSignals {
    pub catnip: RwSignal<f64>,
}

pub struct CalendarSignals {
    pub day: RwSignal<i16>,
    pub season: RwSignal<SeasonKind>,
    pub year: RwSignal<usize>,
}

pub struct StateSignals {
    pub options: OptionSignals,
    pub running_state: RwSignal<RunningState>,
    pub calendar: CalendarSignals,
    pub resources: ResourceSignals,
}

impl StateSignals {
    fn new() -> Self {
        Self {
            options: OptionSignals {
                precision: RwSignal::new(Precision::default()),
            },
            running_state: RwSignal::new(RunningState::default()),
            calendar: CalendarSignals {
                day: RwSignal::new(0),
                season: RwSignal::new(SeasonKind::Spring),
                year: RwSignal::new(0),
            },
            resources: ResourceSignals {
                catnip: RwSignal::new(0.0),
            },
        }
    }

    fn accept(&self, notification: Notification) {
        use Notification::*;

        match notification {
            Initialized => tracing::debug!("World initialized."),
            StateChanged(state) => {
                if let Some(time) = state.time {
                    if let Some(running_state) = time.running_state {
                        self.running_state.set(running_state);
                    }
                }
                if let Some(resources) = state.resources {
                    if let Some(catnip) = resources.amounts.get_state(&Kind::Catnip) {
                        self.resources.catnip.set(*catnip);
                    }
                }
                if let Some(calendar) = state.calendar {
                    if let Some(day) = calendar.day {
                        self.calendar.day.set(day);
                    }
                    if let Some(season) = calendar.season {
                        self.calendar.season.set(season);
                    }
                    if let Some(year) = calendar.year {
                        self.calendar.year.set(year);
                    }
                }
            }
        }
    }
}

pub fn provide_state_signals_context() {
    let signals = Rc::new(StateSignals::new());
    provide_context(SendWrapper::new(signals.clone()));
}

pub fn provide_endpoint_context() {
    let signals = use_state_signals();
    let endpoint = Rc::new(Endpoint::new(
        move |notification| signals.accept(notification),
        "./engine.js",
    ));
    let endpoint_wrapped = SendWrapper::new(endpoint.clone());
    provide_context(endpoint_wrapped);

    endpoint.send(Intent::Load);
}

pub fn use_endpoint() -> SendWrapper<Rc<Endpoint>> {
    use_context::<SendWrapper<Rc<Endpoint>>>().expect("endpoint not provided in context")
}

pub fn use_state_signals() -> SendWrapper<Rc<StateSignals>> {
    use_context::<SendWrapper<Rc<StateSignals>>>().expect("state signals not provided in context")
}

pub fn with_state_signal<S>(
    with: impl Fn(SendWrapper<Rc<StateSignals>>) -> RwSignal<S>,
) -> RwSignal<S> {
    with(use_state_signals())
}
