fn another_function_before_main() {
    println!("Before main");
}

fn main() {

    another_function_before_main();
    println!("Hello, world!");
    another_function_after_main();
    another_function_with_parameters(100, String::from("test"));
    println!("{}", return_funtion());
    
}

fn another_function_after_main() {
    println!("After main");
}

fn another_function_with_parameters(x: i32, y: String) {
    println!("The valu of x from main is : {}", x);
    println!("The valu of y from main is : {}", y);
}

fn return_funtion() -> String {
    return String::from("test");
} 
