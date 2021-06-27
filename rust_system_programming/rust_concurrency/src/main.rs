use std::thread;

fn main() {
    let mut child_threads = Vec::new();
    for _ in 1..5 {
        let handle = thread::spawn(|| {
            println!("Hi from thread id {:?}", thread::current().id());
        });
        child_threads.push(handle)
    }

    for i in child_threads {
        i.join().unwrap();
    }
    concurrency_bulder()
}

fn concurrency_bulder() {
    let mut child_threads = Vec::new();
    for i in 1..5 {
        let builder = thread::Builder::new().name(format!("mythread{}", i));
        let handle = builder
            .spawn(|| {
                println!(
                    "Hi from builder pattern thread id {:?}",
                    thread::current().id()
                );
            })
            .unwrap();
        child_threads.push(handle);
    }

    for i in child_threads {
        i.join().unwrap();
    }
}
