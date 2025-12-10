# Rust LDN - Dec '25 talk

## 1. Crux

- been around about four years, cross-platform “headless” apps with behaviour in Rust and native UI.
- We have support for a growing number of platforms including iOS, Android, Web (native and wasm), and also recently Windows
- Mostly developed by myself and Stu, and more or less in our free time, so the progress is not as fast as we’d love, but the foundations of it work really well and power a number of real world apps that we know of. There’s also a small but active community in Zulip and we do monthly community calls
- The focus now is on amazing DX, until we can be proud to call it 1.0. And the DX is we want to talk about today

## 2. Key to Crux is portability with good DX

- Stu is going to talk about the low-level plumbing of how we make it all work without constantly making you, the developer, aware of low level FFI, pointers and byte buffers
- But I’m going to start us off by showing you the “secret weapon” of Crux - managed effects - and how we made that idea ergonomic

## 3. The problem

- One of the main reasons we started Crux was to be able to use rock solid tools - like Rust – to build Rock solid apps, _once_. And that means portable across platforms.
- It turns out managed effects solve both of those problems, because both of them share the same core problem – we need to intercept any effectful (I/O) code and be able to inspect it, and then resolve it with the result, either using a platform native library, or with a known value for testing

## 4. Look at normal I/O code in Rust - an example with Tokio time

- https://docs.rs/tokio/latest/tokio/time/fn.interval.html#examples
- Really nice and fluent code, with `.await`, it’s clear what’s going on.
- Portability is problematic – the library needs to support the platform or you need a different library. What if you can't find one that supports all your platforms?
- Testing is problematic – if this wasn’t Rust, we would mock or fake things somehow with some horrible hack. In Rust it’s tricky – the rock solid type system stops us, and so we need to change the API somehow to allow multiple implementations. We'd need a trait, or some type of indirection. Most libraries are not written allowing that
- This is a real problem, because in typical apps, especially mobile apps, this sort of thing is often the bulk of the work - deciding what APIs to call in what order, running things based on timers, managing local storage… it’s not a large list of the _kinds_ of things to do, but the app does them A LOT. And testing that kind of higher-level logic, especially in the UI of apps, is super difficult. Most people resort to end to end tests because of that
- With crux, we looked at how this could work if you recognise the effectful interactions as a first-class idea, and come up with an abstraction, that is built with testing and portability in mind from the start.

## 5. Managed effects

- The fundamental problem is that we need to “insert ourselves” in the `.await` and resolve the future with a value of our choosing.
- In rust, this basically means we need to write the code for our own async runtime where we can inspect and control the await points. So the question becomes how we can make that ergonomic
- Our approach is to think of this in a functional programming way (we like pure functions, super easy to understand and test).
- What if instead of a function just using side effects whenever, we had the function _return_ a _recipe_ for the side effects (plural!), and expect the caller to handle them. We can make the effects lazy, and turn them into a value.
- With that we turn an effectful function into a pure one. The rest of the work is designing a nice abstraction for ergonomics and good DX

## 6. Example code

- An egg timer app – runs a clock, we can set the timer by incrementing and decrementing it, run it, paus it, reset it…
- _show the app_
- Crux apps are based on event sourcing, like Elm or Crux, so our entry point is this one `update` function, which takes an event a mutable Model which lets us keep state and returns a `Command` which represents the recipe we just talked about.
- Lets look at the events, the model and the view model, to understand the interface,
- now Instead of reading the code (we can come back to that) lets read the tests - that will help us understand commands and effects
- run the test - runs in under 10ms, clearly no actuall waiting for time is happening
- now lets look at the code quickly – quite ergonomic API (show the `.and`). Don’t have time to go much deeper, but you can easily chain these things, run many of them concurrently, and if you need the full power of async you can have it
- This app also uses a custom capability - Wall Clock, lets look at that - just a nice API to create the commands, but also shows how the effect “message protocol” is defined

Questions? Over to Stu to talk about our evolving codegen support
