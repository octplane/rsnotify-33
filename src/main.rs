extern crate notify;

fn watch(file: &str) -> notify::FsEventWatcher {
    use notify::Watcher;
    use std::sync::mpsc::channel;
    use std::thread;

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let mut watcher = notify::new(tx).unwrap();
    watcher.watch(file).unwrap();

    // Don't join this thread
    thread::spawn(move || {
        loop {
            match rx.recv() {
            Ok(_) => println!("changes!"),
            Err(o) => println!("Err {}", o )
            }
        }
    });

    watcher
}

fn main() {
    // Don't let the watcher drop
    let _ = watch("./test.ares");

    // in my real application, this sleep-loop
    // is actually a webserver that is running
    loop {
        ::std::thread::sleep_ms(20);
    }
}
