# onevent

`onevent` is a WIP event emitter library for Rust, inspired by the [Node.js Event Emitter](https://nodejs.org/docs/latest/api/events.html#class-eventemitter).
The need for this came up while working on a fairly complex project at [Mars](https://www.marscomputers.tech/). I feel like this will help the rest of the community in some way.
Existing implementations do the basic task but I have broader ideas with this project that I will hopefully work through in the coming weeks.

## usage

```rust
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
```

### contributing

I am happy to accept pull requests. No hard rules.

### acknowledgements

created by Vivek Nathani ([@viveknathani_](https://twitter.com/viveknathani_)), licensed under the [MIT License](./LICENSE).
