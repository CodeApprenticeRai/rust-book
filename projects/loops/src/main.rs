fn main() {
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
        
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");


    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    // }
    
    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    // for element in a {
    //     println!("the value is: {element}");
    // }

    // for number in (1..a.len()).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");
    
    // Fahrenheit to Celcius Conversion Example
    // let temp_c: f64 = fahrenheit_to_celcius(72.0);
    // println!("The tmeperature 72.0F is {temp_c}C");
    
    // Calculate nth fibonacci number
    let n: u32 = 5;
    let n_fibonacci: u32 = fibonacci(n);
    println!("The {n}th fibonacci number is {n_fibonacci}");
}

// formula: (32°F − 32) × 5/9 = 0°C
// fn fahrenheit_to_celcius(temp_f: f64) -> f64{
//     return (temp_f - 32.0) * (5.0/9.0);
// }

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n-2) + fibonacci(n-1);
    }
}