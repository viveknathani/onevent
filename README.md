# onevent

`onevent` is a WIP event emitter library for Rust, inspired by the [Node.js Event Emitter](https://nodejs.org/docs/latest/api/events.html#class-eventemitter).
The need for this came up while working on a fairly complex project at [Mars](https://www.marscomputers.tech/). I feel like this will help the rest of the community in some way.
Existing implementations do the basic task but I have broader ideas with this project that I will hopefully work through in the coming weeks.

## usage

```rust
let mut emitter = OnEvent::new();

emitter.on("one", || {
    println!("i like one");
});

emitter.emit("one");
emitter.emit("two");
emitter.emit("one");
```

### Contributing

I am happy to accept pull requests. No hard rules.

### Acknowledgements

created by Vivek Nathani ([@viveknathani_](https://twitter.com/viveknathani_)), licensed under the [MIT License](./LICENSE).
