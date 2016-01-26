use std::time;
use std::thread;


fn main_loop(){
    let CYCLES_PER_SECOND = 1;
    loop {
        println!("loop cycle: {}", loop_cycle_num); 
        loop_cycle_num += 1;
        thread::sleep(time::Duration::from_millis(1000));
    }
}


fn main() {
    println!("Hello, world!");
    main_loop();
}
