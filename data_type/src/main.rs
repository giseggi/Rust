fn main() {
    let test = 98_22;
    println!("test : {}", test);

    let test = 0xff;
    println!("test : {}", test);

    let test = 0o77;
    println!("test : {}", test);

    let test = 0b110110_0011;
    println!("test : {}", test);
    
    let test = 1.2345678901234567890f32;
    println!("test : {}", test);

    let test = 1.2345678901234567890f64;
    println!("test : {}", test);

    let test: f32 = 1.2345678901234567890;
    println!("test : {}", test);

    let test: f64 = 1.2345678901234567890;
    println!("test : {}", test);

    let test = true;
    println!("test : {}", test);

    let test: bool = false;
    println!("test : {}", test);

    let test = '이';
    println!("test : {}", test);

    let test = 'ℤ';
    println!("test : {}", test);

    let test = '😻';
    println!("test : {}", test);

    let test_tup = (10, 12.1, "test");
    let (x, y, z) = test_tup;
    println!("The value of y : {}", y);

    let test_tup: (u32, f32, i32) = (10, 12.1, 100);
    println!("The first data : {}", test_tup.0);

    let month = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The 5th Month is : {}", month[4]);
    
}
