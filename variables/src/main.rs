const MAX_POINTS: u32 = 100_000;

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y; 
}

fn main() {
    println!("Hello, world!");
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let y: i32 = 5;
    let y: i32 = y + 1;
    {
        let y: i32 = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    let space: &'static str = "    ";
    let space: usize = space.len();
    println!("The length of space is: {}", space);
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x: f64 = 2.1; // f64
    let y: f32 = 3.3; // f32
    println!("The value of x is: {}, and the value of y is: {}", x,y);

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of a is: {}, b is: {}, c is: {}", tuple.0, tuple.1, tuple.2);

    //array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The first element is: {}, and the second element is: {}", first, second);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The first month is: {}, and the last month is: {}", months[0], months[11]);
    // let index = 12; // This will cause a panic at runtime
    // let element = a[index];
    // println!("The element at index {} is: {}", index, element);  
    another_function(1, 2);

    let result: i32 = add_numbers(10, 20);
    println!("The result of addition is: {}", result);

    let product: i32 = multiply(5, 6);
    println!("The product is: {}", product);
    five();
    println!("The value returned by five() is: {}", five());

    let number: i32 = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let num: i32 = 6;
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition: bool = true;   
    let number: i32 = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // This will cause a compile-time error
    println!("{}", s2); // This is valid
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let add_value = x + y;
    println!("The value of x + y is: {}", add_value);
}

fn five() -> i32 {
    5
}


