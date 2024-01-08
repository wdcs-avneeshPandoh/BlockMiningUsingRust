use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const NUM_THREADS: usize = 4;

fn generate_random_hash() -> String {

    use rand::Rng;
    rand::thread_rng().gen::<u64>().to_string()
}

fn mine_block(
    target_prefix: &'static str,
    result: Arc<Mutex<Option<String>>>,
    block_counter: Arc<Mutex<usize>>,
    should_continue: Arc<Mutex<bool>>,
) {
    loop {
        let candidate = generate_random_hash();
        let block_number = {
            let mut counter = block_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        println!("Mining block #{} with hash: {}...", block_number, candidate);
        let continue_mining = *should_continue.lock().unwrap();
        if !continue_mining {
            println!("Mining stopped by user.");
            return;
        }

        if candidate.starts_with(target_prefix) {
            let mut result = result.lock().unwrap();
            *result = Some(candidate);
            println!(
                "Block mined successfully! Hash: {}",
                result.as_ref().unwrap()
            );
            break;
        }
    }
}

fn main() {
    let result = Arc::new(Mutex::new(None));
    let block_counter = Arc::new(Mutex::new(0));
    let should_continue = Arc::new(Mutex::new(true));

    println!("Do you want to start mining? (y/n): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let should_continue_value = input.trim().to_lowercase() == "y";
    *should_continue.lock().unwrap() = should_continue_value;

    if !should_continue_value {
        println!("Mining stopped by user.");
        return;
    }

    let mut handles = vec![];
    for _ in 0..NUM_THREADS {
        let result_clone = Arc::clone(&result);
        let block_counter_clone = Arc::clone(&block_counter);
        let should_continue_clone = Arc::clone(&should_continue);
        let handle = thread::spawn(move || {
            mine_block(
                "0000",
                result_clone,
                block_counter_clone,
                should_continue_clone,
            );
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap();
    if let Some(valid_hash) = result.as_ref() {
        println!(
            "Final result: Block mined successfully! Hash: {}",
            valid_hash
        );
    } else {
        println!("Final result: Mining failed. No valid hash found.");
    }
}
