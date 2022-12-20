use {
    clap::Parser,
    serde_yaml::Value,
    std::{
        collections::HashMap,
        fs::File,
        path::{Path, PathBuf},
        time::Instant,
        thread,
    },
    // std::time::Duration,
    crossbeam_channel::bounded,
    std::sync::Arc,
};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// Path to yaml directory
    #[arg(short, long)]
    path: String,
}

#[derive(Debug)]
enum ThreadMsg {
    File(PathBuf),
    Stop,
}

const THREADS_NUM: usize = 2;
const QUEUE_SIZE: usize = 32;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn process_file(file: &PathBuf) -> Result<serde_yaml::Value, Box<dyn std::error::Error> > {
    let reader = File::open(&file)?;
    let value = serde_yaml::from_reader(reader)?;
    Ok(value)
}

fn thread_process_message(
    thread_ind: usize,
    msg: Result<ThreadMsg, crossbeam_channel::RecvError>
) -> bool {
    println!("Thread N{}, Received message: {:?}", thread_ind, msg);
    match msg {
        Ok(ThreadMsg::File(file)) => {
            println!("Thread {} received msg: {:?}", thread_ind, file);
            match process_file(&file) {
                Ok(value) => {
                },
                Err(error) => {
                    println!("Error while processing file {:?}: {:?}", file, error);
                }
            }
            false
        },
        Ok(ThreadMsg::Stop) => {
            println!("Stopping thread {}", thread_ind);
            true
        }
        Err(error) => {
            println!("Error while receiving message: {:?}", error);
            false
        }
    }
}

fn init_threads() -> Vec<(crossbeam_channel::Sender<ThreadMsg>, thread::JoinHandle<()>)> {
    let mut threads_vec = Vec::with_capacity(THREADS_NUM);
    let (tx, rx): (crossbeam_channel::Sender<ThreadMsg>, crossbeam_channel::Receiver<_>) = bounded(QUEUE_SIZE);

    for thread_ind in 0..THREADS_NUM {
        println!("thread_ind: {}", thread_ind);
        let thread_rx = rx.clone();
        let thread_tx = tx.clone();
        let thread = thread::spawn(move || {
            loop {
                let recv = thread_rx.recv();
                let stop = thread_process_message(thread_ind, recv);
                if stop {
                    break;
                }
                let sleep_to = std::time::Duration::from_millis(10);
                thread::sleep(sleep_to);
            }
        });
        threads_vec.push((thread_tx, thread));
    }
    threads_vec
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = std::fs::canonicalize(Path::new(&args.path))?;

    let mut files = HashMap::<PathBuf, Value>::new();

    let now = Instant::now();

    let threads_vec = init_threads();
    let mut ind: usize = 0;

    for file in std::fs::read_dir(path)? {
        let file = file?.path();

        if file
            .extension()
            .map(|ext| ext != "yaml" && ext != "yml")
            .unwrap_or(true)
        {
            continue;
        }
        let msg = file.clone();
        match threads_vec.get(ind) {
            Some((tx, _thread)) => {
                println!("Sending {:?} to thread N{}", file, ind);
                let _ = tx.send(ThreadMsg::File(msg));
                ind += 1;
                if ind >= THREADS_NUM {
                    ind = 1;
                }
            },
            None => {
                println!("Error getting thread from vec: {}", ind);
            }
        }
    }

    ind = 0;
    loop {
        thread::sleep(std::time::Duration::from_millis(10));
    };

    // for (tx, thread) in threads_vec {
    //     let result = tx.send(ThreadMsg::Stop);
    //     println!("Sending result is {:?}", result);
    //     thread.join().unwrap();
    // }

    println!(
        "{} files read in {} secs",
        files.len(),
        now.elapsed().as_secs_f64()
    );

    Ok(())
}
