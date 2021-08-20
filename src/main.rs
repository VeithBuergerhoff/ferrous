use std::time::Instant;

fn main() {
    let now = Instant::now();
    ferrousc_main::run();

    let total_micro = now.elapsed().as_micros();
    let sec = total_micro / 1000_000;
    let millisec = (total_micro as f32 / 1000_f32) % 1000_f32;
    println!("{}:{} sec", sec, millisec);

    ferrousc_main::print();
}
