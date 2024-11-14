use onevent::OnEvent;
use serde::{Deserialize, Serialize};

fn main() {
    let mut emitter = OnEvent::new();

    emitter.on("add", |number: f32| println!("{}", number + 3.0));
    emitter.emit("add", 5.0 as f32);
    emitter.emit("add", 4.0 as f32);

    #[derive(Serialize, Deserialize)]
    struct Options {
        value: f32,
    }
    emitter.on("subtract", |options: Options| {
        println!("{}", options.value - 3.0)
    });
    emitter.emit("subtract", Options { value: 5.0 });
    emitter.emit("subtract", Options { value: 4.0 });
}
