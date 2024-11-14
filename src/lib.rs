use bincode;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

pub struct OnEventListener {
    callback: Arc<dyn Fn(Vec<u8>) + Sync + Send + 'static>,
}

/// This is the main entity for managing events in your application.
/// Each instance will maintain its list of listeners and trigger them on demand.
pub struct OnEvent {
    listeners: HashMap<String, Vec<OnEventListener>>,
}

impl OnEvent {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// Registers an event listener with a callback that takes serialized data.
    pub fn on<F, T>(&mut self, event: &str, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Sync + Send,
    {
        // Wrap the callback to handle deserialization from `Vec<u8>`
        let parsed_callback = move |bytes: Vec<u8>| {
            let value: T = bincode::deserialize(&bytes).unwrap();
            callback(value);
        };

        self.listeners
            .entry(event.to_string())
            .or_default()
            .push(OnEventListener {
                callback: Arc::new(parsed_callback),
            });
    }

    /// Emits an event, serializing the provided arguments.
    pub fn emit<T>(&self, event: &str, args: T)
    where
        T: Serialize,
    {
        if let Some(listeners) = self.listeners.get(event) {
            // Serialize the arguments to `Vec<u8>`
            if let Ok(serialized_args) = bincode::serialize(&args) {
                for listener in listeners {
                    let callback = Arc::clone(&listener.callback);
                    callback(serialized_args.clone());
                }
            } else {
                eprintln!("Failed to serialize arguments for event `{}`", event);
            }
        }
    }
}
