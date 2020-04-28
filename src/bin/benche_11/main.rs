


// So, what we want to do in this 'lab bench' kind of thing, is start a process,
// this process will load terrain into memory according to the procedure
// we've implemented in Peregrine.  Then once that's started, we'll start another process,
// via a distinct cargo command, and this process needs to load the terrain somehow from the
// first process, and then render it.
// If we're successful then we can save tons of time in development, by separating costly terrain
// generating procedures from the main developmet editing loop.  This will also come in handy
// in a real program by speeding up for example certain scenario reload situations.


// The fastest way to prove this would be to get an Arc or something similar setup with some kind of buffer
// then just immediately try to connect another process and share the reference.
// The rest is already demonstrated.



fn main() {
    println!("benche 11");
}
