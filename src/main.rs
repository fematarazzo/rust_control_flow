fn main() {
    convert_celsius_to_farenheit(10);
    convert_farenheit_to_celsius(30);
    print_fib(12);
}

fn convert_celsius_to_farenheit(temp_celsius: i32) {
    let temp_farenheit = (temp_celsius * 9 / 5) + 32;
    println!("The temperature in Farenheit is {temp_farenheit}ºF");
}

fn convert_farenheit_to_celsius(temp_farenheit: i32) {
    let temp_celsius = (temp_farenheit - 32) * 5 / 9;
    println!("The temperature in Celsius is {temp_celsius}ºC");
}

fn fibonacci_number(number: u32) -> u32 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    } else {
        return fibonacci_number(number - 1) + fibonacci_number(number - 2);
    }
}

fn print_fib(n: u32) {
    let number = fibonacci_number(n);
    println!("The Nth Fibonacci number is {number}");
}
