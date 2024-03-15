use std::io::{self};

// function to run find fibonacci name
fn fib() {
    let mut target = String::new();

    io::stdin()
        .read_line(&mut target)
        .expect("This needs to be a number");

    let target: u32 = match target.trim().parse() {
        Ok(target) => target,
        Err(_) => {
            println!("Error");
            return;
        }
    };

    println!("nth number: {target}");

    // i need two variables and update them
    let mut a = 0;
    let mut b = 1;
    let mut res = 0;

    // 0 + 1 + 1 + 2 + 3 ...

    for _num in 0..target - 1 {
        // println!("{res}");
        res = a + b;
        a = b;
        b = res;
    }

    println!("Result: {res}");
}

// TODO: add function for The Twelve Days of Christmas

fn sing_song() {
    // the outer loop needs to run 12 times.
    // three sentences repeat throughout the song.
    // one needs string concat. on the {numDay} of christmas
    // my true love gave me. always comes after
    // A partridge in a pear tree. (just on first iteracion.)
    // And a partridge in a pear tree. (On the remaining iterations).

    let nth_months = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let month_phrase = [
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];


    for month_num in 0..11 {
        println!("On the {} day of Christmas", nth_months[month_num]);
        println!("My true love gave me");

        for phrase_num in (0..month_num).rev() {

            println!("{}", month_phrase[phrase_num]);
            
        }
         
        match month_num == 0 {
            true => println!("A partridge in a pear tree."),
            false => println!("And a partridge in a pear tree."),
        }
        println!();
    }
}

fn temp_converter() {
    let mut f_temp = String::new();

    println!("What is fahreheit temp?");

    io::stdin().read_line(&mut f_temp).expect("Needs to be a number");

    let f_temp: f32 = match f_temp.trim().parse() {
        Ok(f_temp) => f_temp,
        Err(_) => {
            print!("There has been a mistake");
            return;
        }
    };

    let celsius = (5.0 / 9.0) * (f_temp - 32.0);

    println!("temp in celsius is: {celsius}");
}

fn main() {

    
    loop {
        // is very important to declare a new string. If left outside loop, `select` is not empty, and future inputs just append to previous values.
        let mut select = String::new();
        println!("What program do you want to run? (SELECT A NUMBER)");
        println!("1. nth fib number");
        println!("2. sing song");
        println!("3. temp converter (F to C)");
        println!("4. EXIT");

        io::stdin().read_line(&mut select).expect("Something went wrong");


        let select = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("There was an error");
                return;
            }
        };


        match select { 
            1 => fib(),
            2 => sing_song(),
            3 => temp_converter(),
            4 => break,
            _ => {
                println!("Select a number from 1 to 4");
                return;
            }
        };

    }

}
