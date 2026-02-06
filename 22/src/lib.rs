#![allow(static_mut_refs)]

// Treat ourselves to a kb (1024 bits)
// 1024 >> 3 == 128 == 0x80
pub const SIZE: usize = 0x80;

// Not really a BUS but we gotta call it something.
static mut BUS: [u8; SIZE] = [0u8; SIZE];

// Zero the array except the mask.
fn init() {
    unsafe {
        // Initialize mask
        // The following explodes if SIZE isn't a power of 2
        assert!(SIZE & (SIZE - 1) == 0);
        // First SIZE >> 3 bits are reserved as a validty byte/bit mask
        let mask_len: usize = SIZE >> 3;
        // Which has to reserve enough bytes for itself.
        // Set to 1
        let mut x: usize = 0;
        while x < mask_len {
            BUS[x / 8] |= 1 << x % 8; // go to byte location and shift to bit, e.g. 12th bit is go to 1st byte and shift 4x
            x += 1;
        }
        // Initialize memory
        // Set to zero.
        println!("{:x?}", BUS);
    }
    return;
}

// Return an index in BUS of s reserved bytes
pub fn malloc(s: usize) -> Option<usize> {
    unsafe {
        // Ensure BUS is initialized.
        if BUS.iter().all(|&x| x == 0) {
            init();
            println!("^^ init ^^");
        }
        // Reserve a block of s bytes
        let mut bit: usize = 0;
        while bit < SIZE {
            // loop through all bits (SIZE # of bits = bitmask)
            if (BUS[bit / 8] & 1 << bit % 8) == 0 {
                // if the bit is available:
                let mut s_check: usize = 1;
                while s_check < s {
                    // check the next s-1 bits if they are available too
                    if (bit + s_check) / 8 == SIZE {
                        // Prevent index oob
                        return None;
                    }
                    if (BUS[(bit + s_check) / 8] & 1 << (bit + s_check) % 8) != 0 {
                        // if fails any, break
                        break;
                    } else {
                        s_check += 1;
                    }
                }
                if s_check == s {
                    // if s_check reached s, then all s bits must have been free
                    println!("Success, {}", s);
                    s_check = 0;
                    while s_check < s {
                        BUS[(bit + s_check) / 8] |= 1 << (bit + s_check) % 8;
                        s_check += 1;
                    }
                    println!("{:x?}", BUS);
                    return Some(bit);
                }
            }
            bit += 1; // can jump at steps of s_check
        }
        // Scan for a contigious region of size s
        //
        // In s > 8, word level allocatioit)       // "Could be more efficient" it's an exercise!
        println!("{:x?}", BUS);
    }
    println!("Fail, {}", s);
    return None;
}

// Place val at loc
// No safety checks so good luck out there.
pub fn setter<T>(val: T, loc: usize) {
    unsafe {
        let ptr: *const T = &val as *const T;
        let val: *const u8 = ptr.cast::<u8>();
        let bytes: usize = size_of_val(&val);
        println!("{}", val as u8);
        let mut x: usize = 0;
        while x < bytes {
            BUS[loc + x] = val as u8;
            x += 1;
        }
        println!("check set {:x?}", &BUS);
    }
    return;
}

/* Should check the validity bitmask here...
pub fn getter<T>(loc: usize) -> T {
    unsafe {
        return 1;
    }
}
*/
