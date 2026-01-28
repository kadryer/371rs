fn main() {
    unsafe {
        let signed32s: [i32; 12] = [
            72i32, 101i32, 108i32, 108i32, 111i32, 32i32, 119i32, 111i32, 114i32, 108i32, 100i32,
            33i32,
        ];
        let letters: [char; 12] = std::mem::transmute(signed32s);
        let string: String = letters.iter().collect();
        println!("{}", string)
    }
}
