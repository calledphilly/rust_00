use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

// mod list;

fn sleep(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}
macro_rules! sleep_and_lock {
    ($duration:expr, $clone:expr, $guard:ident, $block:block) => {{
        sleep($duration);
        if let Ok(mut $guard) = $clone.lock() {
            $block
        }
    }};
}

fn sleep_range(duration: u64) {
    thread::sleep(Duration::from_millis(rand::random_range(0..=duration)));
}
macro_rules! sleep_range_and_lock {
    ($duration:expr, $clone:expr, $guard:ident, $block:block) => {{
        sleep_range($duration);
        if let Ok(mut $guard) = $clone.lock() {
            $block
        }
    }};
}

macro_rules! lock {
    ($clone:expr, $guard:ident, $block:block) => {{
        if let Ok(mut $guard) = $clone.lock() {
            $block
        }
    }};
}

fn main() {
    let mut handlers = Vec::new();
    let vec: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));

    for id in 0..=5 {
        let vec_cloned = Arc::clone(&vec);
        handlers.push(thread::spawn(move || {
            sleep_range_and_lock!(3000, vec_cloned, guard, {
                guard.push(id);
            });
            sleep_range_and_lock!(3000, vec_cloned, guard, {
                guard.push(id);
            });
        }));
    }
    for h in handlers {
        h.join().unwrap();
    }

    {
        let vec_cloned = Arc::clone(&vec);
        thread::spawn(move || {
            if let Ok(mut guard) = vec_cloned.lock() {
                guard.pop();
            }
        });
    }

    thread::scope(|s| {
        let vec_cloned = Arc::clone(&vec);
        s.spawn(move || {
            lock!(vec_cloned, vec_guard, {
                vec_guard.push(20);
            });
        });
    });
    println!("{:?}", vec.lock().unwrap());
}
