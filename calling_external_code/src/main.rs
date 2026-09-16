static mut COUNTER: u32 = 0;

fn main() {
    let negative_number = -32;

    println!(
        "Absolute value of {} according to C is {}",
        negative_number,
        abs(negative_number)
    );

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        for _ in 1..=10000 {
            add_to_count(3);
        }
        println!("COUNTER = {}", COUNTER);
    }
}

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
