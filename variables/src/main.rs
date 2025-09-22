const MAX_POINTS: u32 = 100_000;
fn main() {
    println!("Hello, world!");
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    let space = "    ";
    let space = space.len();
    println!("The length of space is: {}", space);
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
