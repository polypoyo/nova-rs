# Async Await Primer

Futures, rust nightly, async/await? What is all this modern language
bullshit? It is the core of how to build usable parallel programs.
It is an extension of task-based parallelism: splitting the program into
many different interdependent tasks that depend on each other. In a
language without coroutines, it is often a lot of work to get this to
work because there is no real way to stop an operation in place. Rust
does not easily expose their coroutine library (to my knowledge) but
does have the async/await syntax that allows for easily making all code
asynchronous.

Additionally, futures v0.3 (provided by the `futures-preview` crate), is
generally not meant to be worked with directly, it is meant to be used
with async/await. Because of this, you shan't worry.

Warning: Async, like the GPL, is called "infectious" as it is hard (ish)
to call async code from non-async code. This isn't nearly as big of a
problem for us, as everything will most likely be going in the other
direction async->sync.

### Rust Nightly

In order to use the async/await syntax you have to enable rust nightly.
This toolchain is one of the three main ones available (`stable`/`beta`/
`nightly`). This seems scary, wouldn't nightly be very unstable? Unlike
a lot of other compilers, you have to enable the unstable features
manually, so there isn't much chance the world is going to explode into
a large collection of nasal daemons (either way we left those back in
c++ land). 

## Baby's First Async

I think the best way to explain this is to cut through the bullshit and
just make a simple async example. It will be fully commented to explain
what is happening.

```rust
#![feature(async_await)] // Enable async/await only available in rust nightly

use futures::executor::ThreadPool;        // Thread pool to run everything on.
use futures::executor::ThreadPoolBuilder; // Builder for ^
use futures::task::SpawnExt;              // Trait we need in scope for
                                          // ThreadPool::spawn_with_handle

// Simple async function
async fn quick_maths(lhs: f32, rhs: f32) -> f32 {
    lhs * rhs
}

async fn thicc_maths(lhs: f32) -> f32 {
    lhs.sin()
}

// Functions that want to spawn more tasks should have a handle to
// the thread pool (or your chosen executor).
async fn double_maths(mut tp: ThreadPool, lhs: f32, rhs: f32) -> f32 {
    // "Calling" an async function doesn't actually call it. It gives
    // us a future that we can await on later. This also doesn't
    // actually start the future executing. It must first be given to
    // an executor which will do the executing.
    let future = quick_maths(lhs, rhs);

    // This starts the job running on the thread pool.
    let remote_handle = match tp.spawn_with_handle(future) {
        // This is a "handle" to the remote execution, this is also a future,
        // i.e. something we can await on.
        Ok(handle) => handle,
        // This is often handles through an .expect, .unwrap, or ?, but I wanted to be
        // clear about the types by doing a full match
        Err(_err) => panic!("Why you no spawn")
    };

    // Wait for the future to be done and get the value. If the future
    // is not done, this function pauses in place, freeing the thread to
    // go do other things. Through "magic" this will resume once the
    // value is ready. This is a proper coroutine. Control returns to the
    // executor running this function (double_maths).
    let quick = remote_handle.await;

    // Poof, all at once.
    let slow = tp.spawn_with_handle(thicc_maths(quick)).unwrap().await;

    slow
}

fn main() -> std::io::Result<()> {
    let mut pool = ThreadPoolBuilder::new().create()?;

    // We cannot pass the thread pool into the async function as is due
    // to borrow checker rules, so we "clone" it. This is just a shallow
    // increment of an Arc, so is cheap.
    let clone = pool.clone();

    // Run takes a single async function as the "main" function in the pool.
    // This will block until it terminates. Remember we are passing the future
    // into the pool as "calling" an async function just gives you a future.
    let result = pool.run(double_maths(clone, 4.0, 5.0));

    assert_eq!(result, 0.9129453);

    Ok(())
}

// Async functions are really just syntactic sugar for a whole-function async block
// while also hiding the actual true return type. They are quite nice. These two
// functions are identical.
async fn async_other_func() -> i32 {
    3 * 3
}

fn other_func() -> impl futures::Future<Output = i32> {
    async {
        3 * 3
    }
}
```

## Limitations

Known limitations for async code are as follows:

Because the async code gets turned into a state machine, it cannot
recurse at all. This is a smaller problem then you'd think. If you need
to use recursion to do work, you should be doing that work in one single
async task, not splitting it up into many, so you can have your async
function as a "wrapper" to the synchronous work.

## Performance

Based on a really stupid "async bomb" test, where one task spawns 10 more,
which spawns 10 more, etc. There can be about 1.5M spawn/await pairs per
second. This translates to about 25K per frame. This seems like a small
amount, but is most likely adequate for most things.

## Other Things

There is more to the async/await topic, I will add more as they become
relevant or I discover them myself.
