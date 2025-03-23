use rand::Rng;
// use std::num::NonZeroU8;
// use std::thread;
use crate::GenPassError;
use std::{
    fs,
    io::{self, BufWriter, Write},
    num::NonZeroU8,
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Mutex},
    thread::{self},
    time::Duration,
};
//use tracing::trace;

pub fn generate_passwords(
    chars: u8,
    out_file: &Option<String>,
    num_threads: NonZeroU8,
    gen_num: usize,
) -> Result<(), GenPassError> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~ ";
    let mut handles = vec![];
    //let pass_per_thread = gen_num / num_threads.get() as usize; // get is used to take inner value of NonZero type as primitive type

    let counter = Arc::new(AtomicUsize::new(0));

    let generated_passwords: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..num_threads.get() as usize {
        let generated_passwords = Arc::clone(&generated_passwords);
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut rng = rand::rng();

            println!("thread {:?} generated", thread::current());

            loop {
                // sleep for some random time.
                thread::sleep(Duration::from_millis(rng.random_range(1..500)));

                let generate_password = (0..chars as usize)
                    .map(|_| {
                        let index = rng.random_range(0..CHARSET.len());
                        CHARSET[index]
                    })
                    .collect::<Vec<_>>();

                // fetch_add adds 1 to counter and return prevoius value
                let inc = counter.fetch_add(1, Ordering::SeqCst);

                //checks if all required numbers are processed
                if inc >= gen_num {
                    break;
                }

                // if generated_passwords.len() >= gen_num {
                //     break;
                // }

                let mut generated_passwords = generated_passwords.lock().unwrap();
                generated_passwords.extend(String::from_utf8(generate_password));

                println!(
                    "thread {:?} received {} size {}",
                    thread::current(),
                    generated_passwords.len(),
                    gen_num
                );

                // let mut dec = counter.lock().unwrap();
                // *dec -= 1;
                // if *dec !=0 {
                //     break;
                // }

                // for _ in 0..pass_per_thread {
                //     let generate_password: String = (0..chars as usize)
                //         .map(|_| {
                //             let index = rng.random_range(0..CHARSET.len());
                //             CHARSET[index] as char
                //         })
                //         .collect();
                //     thread_output.push(generate_password);
                // }
                // let mut generated_passwords = generated_passwords.lock().unwrap();
                // generated_passwords.extend(thread_output);
            }
        });
        handles.push(handle)
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => (),
            Err(e) => {
                return Err(GenPassError::ThreadJoin(format!(
                    "generate_passwords() {e:?}"
                )));
            }
        }
    }

    let generated_passwords = generated_passwords.lock().unwrap();
    match out_file {
        Some(out_file) => {
            let file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&out_file)?;

            let mut file = BufWriter::new(file);

            for password in generated_passwords.iter() {
                writeln!(file, "{}", password)?;
            }
        }
        None => {
            let mut file = BufWriter::new(io::stdout());
            for password in generated_passwords.iter() {
                writeln!(file, "{}", password)?;
            }
        }
    }

    Ok(())
}
