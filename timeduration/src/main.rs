use std::time::SystemTime;
use std::thread;
use std::time::Duration;
fn main() {
	let before_time = SystemTime::now();
	thread::sleep(Duration::from_millis(1000));
	let five_seconds = Duration::new(5, 0);
	let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);
	thread::sleep(five_seconds_and_five_nanos);
	thread::sleep(Duration::from_secs(1));
	thread::sleep(Duration::from_nanos(1));
	thread::sleep(Duration::from_micros(1));
	let can_time = SystemTime::now();
        let difference = can_time.duration_since(before_time);
        println!("Hello, world!{:?}",difference);
}
