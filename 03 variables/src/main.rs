fn main() {
    another_function();
    print_labeled_measurement(5, 'h');

    let x = plus_one(5);
    println!("x = {x}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    if_example();
    loop_example();
    metki();
    while_example();
    for_example();
    range_example();

}

fn loop_example() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}


fn metki() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn if_example() {

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn range_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function() {
    println!("Another function.");
       // let mut  x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    //
    let x = 5;
    let x = x + 1;

    {
        // Затеняем переменную x в данной области видимости
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (x, y, _) = tup;

    let z = tup.2;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}
