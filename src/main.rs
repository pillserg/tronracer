#![feature(time2)]
use std::time;
use std::thread;


const NANOS_PER_SEC: u64 = 1_000_000_000;
const NANOS_PER_MILLI: u64 = 1_000_000;
const MILLIS_PER_SEC: u64 = 1_000;


fn main_loop(){
    let cycles_per_second = 1;
    let loop_time_cap: u64 = 1000 / cycles_per_second;
    let mut loop_cycle_num = 0;
    loop {
        println!("loop cycle: {}", loop_cycle_num); 
        let start_time = time::Instant::now();

        loop_cycle_num += 1;
        thread::sleep(time::Duration::from_millis(100)); 
        let delta_time = start_time.elapsed();
        let delta_ms = delta_time.as_secs() * MILLIS_PER_SEC + delta_time.subsec_nanos() as u64 / NANOS_PER_MILLI;
        
        let time_left = loop_time_cap - delta_ms;
        if time_left > 0 {
            thread::sleep(time::Duration::from_millis(time_left));    
        }
        println!("{:?}", delta_ms);
    }
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", time::Instant::now());
    main_loop();
}
