use std::{thread, time::Duration};

pub fn remainings(delay:u32) {
    for remaining in (0..=delay).rev() {
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        println!("Countdown: {:02}:{:02}", minutes, seconds); // Format MM:SS
        thread::sleep(Duration::from_secs(1));
    }
}