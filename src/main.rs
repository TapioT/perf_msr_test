#![feature(asm)]
#[cfg(target_arch = "x86_64")]

fn read_tsc() -> u64 {
    let high: u32;
    let low : u32;
    unsafe {
        asm!("rdtsc;" 
            : "={eax}" (low), "={edx}" (high)
            :
            : "%eax", "%edx")
    }
    let mut high_long = high as u64;
    high_long <<= 32;
    let low_long = low as u64;
    high_long | low_long   
}

/*
// This does not work as such
fn read_msr(m: u32) -> i64 {
    let edx: u32;
    let eax: u32;
    unsafe {
        asm!("rdmsr;"
            : "={edx}" (edx), "={eax}" (eax)
            : "%ecx" (m)
            :)
     }
    let mut high = edx as i64;
    high <<= 32;
    let low = eax as i64;
    return high | low
}
*/

fn read_cpuid(eax_c : u32) -> (u32, u32, u32, u32) {
    let eax : u32;
    let ebx : u32;
    let ecx : u32;
    let edx : u32;    

    unsafe {
        asm!("cpuid;"
            : "={eax}" (eax), "={ebx}" (ebx), "={ecx}" (ecx), "={edx}" (edx)
            : "{eax}" (eax_c)
            :)
    }
    (eax, ebx, ecx, edx)
}

fn u8bytes2u64( bytes : [u8; 8]) -> u64 {
     bytes[0] as u64 + 
    ((bytes[1] as u64) <<  8) +
    ((bytes[2] as u64) << 16) + 
    ((bytes[3] as u64) << 24) + 
    ((bytes[4] as u64) << 32) +
    ((bytes[5] as u64) << 40) +
    ((bytes[6] as u64) << 48) +
    ((bytes[7] as u64) << 56)	
}



fn read_msr2(m: u64, cpu: i32) -> Result< u64, std::io::Error> {
    use std::fs::File;
    use std::io::SeekFrom;
    use std::io::prelude::*;
    use std::path::Path;
    
    let msr_file_name = format!("/dev/cpu/{}/msr", cpu);
    let msr_file_path = Path::new(&msr_file_name);
    let mut f = try!(File::open(&msr_file_path));
    
    let seek_result = try!(f.seek(SeekFrom::Start(m)));
    
    assert_eq!(m, seek_result);   
    let mut bytes : [u8; 8] = [0;8];
    let bytes_read = try!(f.read(&mut bytes));
    assert_eq!(bytes_read, 8);
    Ok(u8bytes2u64(bytes))
}


fn main() {
    foo();
    println!("Hello, world!");
    for i in 0..10 {
        println!("{} Tsc today is {}", i, read_tsc());
    }
    let b = read_msr2(0x10, 0).unwrap();
    println!("Same with rdmsr: {}", b);
    let r : (u32, u32, u32, u32) = read_cpuid(0);
    println!("Called cpuid with eax=0, got {}, {}, {}, {}", r.0, r.1, r.2, r.3)
}


#[test]
fn test_read_tsc() {
    let a : u64 = read_tsc();
    let b : u64 = read_tsc();
    assert!(b > a);
    assert!( b-a < 10000);
}

#[test]
fn test_read_msr2() {
    // 0x10 is the msr for reading tsc (ie, rdtsc), so this is a good comparison
    let a = read_tsc();
    let b = read_msr2(0x10, 0).unwrap();
    let c = read_tsc();
    assert!( b > a ); 
    assert!( c > b );
    assert!( c > a );
}

#[test]
fn test_cpuid_0() {
    // Sanity check. These are constant for all Intel processors
    let r = read_cpuid(0);
    assert_eq!(r.1, 0x756e6547);
    assert_eq!(r.3, 0x49656e69);
    assert_eq!(r.2, 0x6c65746e);
}

#[test] 
fn test_u8bytes2u64() {
    assert_eq!(1, u8bytes2u64([1, 0, 0, 0, 0, 0, 0, 0]));	
    assert_eq!(72057594037927936, u8bytes2u64([0, 0, 0, 0, 0, 0, 0, 1]));	
}







