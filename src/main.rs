use onevent::OnEvent;

fn main() {
    let mut emitter = OnEvent::new();

    emitter.on("one", || {
        println!("i like one");
    });

    emitter.emit("one");
    emitter.emit("two");
    emitter.emit("one");
}
