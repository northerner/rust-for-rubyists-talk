use std::thread;
fn main() {
    let mut children = vec![];

    for i in 0..10 {
        children.push(thread::spawn(|| {
            println!("this is thread number {}", i)
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
