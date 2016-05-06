#![feature(asm)]
#[cfg(target_arch = "x86_64")]

fn foo() {
    unsafe {
        asm!("NOP");
    }
}

fn read_tsc() -> i64 {
    let mut high: u32;
    let mut low : u32;
    unsafe {
        asm!("rdtsc;" 
            : "={eax}" (low), "={edx}" (high)
            :
            : "%eax", "%edx")
    }
    let mut high_long = high as i64;
    high_long <<= 32;
    let low_long = low as i64;
    high_long | low_long   
}


fn main() {
    foo();
    println!("Hello, world!");
    for i in 0..10 {
        println!("Tsc today is {}", read_tsc());
    }
}
