use {
    std::{
        // collections::HashMap,
        time::Instant,
        thread,
    },
    crossbeam_channel::bounded,
    // std::sync::Arc,
};

#[derive(Debug)]
enum ThreadMsg {
    Cmd(String),
    Stop,
}

fn spawn_thread(thread_n: usize, rx: &crossbeam_channel::Receiver<ThreadMsg>) {
    let rx_ = rx.clone();
    let _thread = thread::spawn(move || {
        loop {
            let sleep_to = std::time::Duration::from_millis(1000);
            match rx_.recv() {
                Ok(cmd) => {
                    println!("=> N{}: OK Received: {:?}", thread_n, cmd);
                },
                Err(err) => {
                    println!("=> N{}: Err Received: {:?}", thread_n, err);
                }
            };
            thread::sleep(sleep_to);
        }
    });
}

fn main() {
    let (tx, rx): (crossbeam_channel::Sender<ThreadMsg>, crossbeam_channel::Receiver<_>) = bounded(1);
    let now = Instant::now();

    spawn_thread(1, &rx);
    spawn_thread(2, &rx);

    loop {
        let elapsed = now.elapsed().as_secs_f64().to_string();
        println!("=> MAIN: Send: {:?}", elapsed);
        match tx.send(ThreadMsg::Cmd(elapsed)) {
            Ok(res) => {
            },
            Err(err) => {
                println!("!!!! => MAIN: Err Send result: {:?}", err);
            }
        }
        // let sleep_to = std::time::Duration::from_millis(10);
        // thread::sleep(sleep_to);
    };
}
