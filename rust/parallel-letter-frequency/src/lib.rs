use crossbeam_utils::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }
    if worker_count == 0 {
        panic!("workers must be > 0");
    }

    let mut chunk_size = input.len() / worker_count;
    if input.len() % worker_count != 0 {
        chunk_size += 1;
    }

    let total_count = Arc::new(Mutex::new(HashMap::new()));
    thread::scope(|s| {
        let mut pool = Vec::with_capacity(worker_count);

        for chunk in input.chunks(chunk_size) {
            let total_count = Arc::clone(&total_count);
            pool.push(s.spawn(move |_| {
                let mut thread_count = HashMap::new();
                for string in chunk {
                    for c in string.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                        *thread_count.entry(c).or_insert(0) += 1;
                    }
                }
                let mut total_count = total_count.lock().unwrap();

                for (letter, count) in thread_count {
                    *total_count.entry(letter).or_insert(0) += count;
                }
            }));
        }
        for thread in pool {
            thread.join().unwrap();
        }
    })
    .unwrap();
    Arc::try_unwrap(total_count).unwrap().into_inner().unwrap()
}
