use std::collections::HashMap;

pub struct OnEventListener {
    pub callback: Box<dyn Fn() + Send + Sync + 'static>,
}

#[derive(Default)]
pub struct OnEvent {
    listeners: HashMap<String, Vec<OnEventListener>>,
}

/// This is the main entity for managing events in your application.
/// Each instance will maintain its list of listeners and trigger them on demand.
impl OnEvent {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn on<F>(&mut self, event: &str, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.listeners
            .entry(event.to_string())
            .or_default()
            .push(OnEventListener {
                callback: Box::new(callback),
            });
    }

    pub fn emit(&mut self, event: &str) {
        let listeners = self.listeners.get(event);
        if listeners.is_some() {
            for listener in listeners.unwrap() {
                (listener.callback)();
            }
        }
    }
}
