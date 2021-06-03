// mod temperature;
// mod fibonacci;
// mod christmas;

// mod guess_the_number_game;

// mod first_word;

// mod structs;

// mod coins;
// use coins::{get_value, Coin, USState};

// mod stats;
// mod pig_latin;
// mod employee; // not finished

fn main() {
    println!("Hello, world!\n");

    // let f = 85.3;
    
    // let c = temperature::f_to_celsius(f);
    // let f = temperature::c_to_fahrenheit(c);

    // println!("{}f = {}c", f, c);


    // let nth = fibonacci::nth_fibonacci(42);
    // println!("{}", nth);

    // christmas::twelve_days_christmas();


    // guess_the_number_game::run();


    // let hi = String::from("Hi Abdullah!");
    // println!("{}", first_word::func(&hi[..]));

    // let hey = "Hey Abdullah!";
    // println!("{}", first_word::func(hey));


    // let user1 = structs::User::new_user(String::from("example@gmail.com"), true, String::from("red"));
    // println!("User 1's description:\n{}", user1.get_desc());

    // let user1 = structs::User {
    //     fav_color: String::from("blue"),
    //     ..user1
    // };

    // println!("\nUser 1's new description:\n{}", user1.get_desc());


    // let coin = Coin::Penny;
    // let val = get_value(coin);
    // println!("{}", val);

    // let coin = Coin::Quarter(USState::Alaska);
    // let val = get_value(coin);
    // println!("{}", val);

    
    // mod is used to start a module or import one
    // Most imported stuff is private (except Enum parts, etc??),
    // and u can make them pub, by using the keyword: pub

    
    // let vec: Vec<i32> = vec![17, 69, 3, 3, 26, 98, 3, 99, 70, 69, 11];
    // let (avg, median, mode) = stats::get_central_tendency(&vec);
    // println!("Avg: {} Median: {} Mode: {}", avg, median, mode);


    // let strings = vec![String::from("Apple"), String::from("First")];
    // let pig_latin = pig_latin::func(&strings);
    // for string in &pig_latin {
    //     println!("{}", string);
    // }
}