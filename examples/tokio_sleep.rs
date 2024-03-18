// FROM HERE last answer
// https://stackoverflow.com/questions/71764138/how-to-run-multiple-tokio-async-tasks-in-a-loop-without-using-tokiospawn


use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Cell is an acceptable complication when accessing the data.
    let val = std::cell::Cell::new(1);
    tokio::select! {
      _ = async {loop {
        println!(".{}", val.get());
        sleep(Duration::from_millis(200)).await;
      }} => {},
      _ = async {loop {
        println!("Starting slow operation...");
        // The problem: During this await the dots are not printed.
        sleep(Duration::from_secs(1)).await;
        val.set(val.get() + 1);
        println!("...done");
        sleep(Duration::from_secs(3)).await;
      }} => {},
    }
}
