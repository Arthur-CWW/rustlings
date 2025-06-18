// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

#[derive(Clone)]
struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let mut status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            // Arc::make_mut().
            // Arc::make_mut(status_shared);
            // status_shared.jobs_done += 1;
            // status_shared.().unwrap();
            // let mut num = status_shared.lock().unwrap();
            Arc::make_mut(&mut status_shared).jobs_done += 1;
            // status_mut.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    // for handle in handles {

    // }

    // TODO: Print the value of `JobStatus.jobs_done`.
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Jobs done: {}", status.jobs_done);
}
