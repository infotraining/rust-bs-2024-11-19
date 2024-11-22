mod thread_pool;

use std::{sync::mpsc, thread::JoinHandle};

use thread_pool::ThreadPool;

fn simple_multithreading() {
    let mut thd_handles = Vec::<JoinHandle<()>>::new();

    for i in 1..5 {
        let builder = std::thread::Builder::new().name(format!("THD#{i}"));
        let thd_handle = builder
            .spawn(move || {
                for j in 1..10 {
                    println!(
                        "Hi - step#{} - {}",
                        j,
                        std::thread::current().name().unwrap()
                    );
                    std::thread::sleep(std::time::Duration::from_millis(200 * i));
                }
            })
            .unwrap();

        thd_handles.push(thd_handle);
    }

    for handle in thd_handles {
        handle.join().unwrap();
    }
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    std::thread::spawn(move || {
        for i in 1..5 {
            let val = format!("Hi -  {}", i);
            tx1.send(val).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    std::thread::spawn(move || {
        for i in 1..10 {
            let val = format!("Hello -  {}", i);
            tx.send(val).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn thread_pool_demo() {
    let thd_pool = ThreadPool::new(4);

    for id in 1..10 {
        thd_pool.execute(move || {
            println!("Task#{}", id);
        })
    }
}

fn main() {
    //simple_multithreading();

    println!("--------------");

    //message_passing();

    println!("--------------");

    thread_pool_demo();
}
