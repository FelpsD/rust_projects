fn main() {
    let guess: usize = 0xffffffffffffffff; //arch length (64bit) in hex

//  let guess2: u16 = b'A'; //expected `u16`, found `u8`
    let guess2: u8 = b'A';

    println!("Guess: {guess}!");
    println!("Guess 2: {guess2}");

// The default type is f64 because on modern CPUs, 
// it’s roughly the same speed as f32 but is capable of more precision. 
// All floating-point types are signed.

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let f: bool = false; // with explicit type annotation
    let t = true;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    println!("The value of tup y: {0}", tup.1);

//  Array has fixed length and the same type
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];

}
