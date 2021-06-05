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

// mod read_username;

// mod generics;

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

    
    // let username = match read_username("txt") {
    //     Ok(username) => username,
    //     Err(e) => panic!("Error: {:?}", e)
    // };
    // let username = std::fs::read_to_string("txt").unwrap();
    
    // println!("{}", username);

    
    // let (v1, v2) = (10, 100);
    // println!("{}", generics::larger(v1, v2));

    // let (str1, str2) = ("str1 short", "str2 looooooooooooooong");
    // println!("{}", generics::longer(str1, str2));

    // let pair1: generics::Pair<i32, f64> = generics::Pair { item1: 1, item2: 9.99 };
    // let pair2: generics::Pair<char, char> = generics::Pair { item1: 'f', item2: 'f' };
    // let pair3 = generics::Pair::mixup(&pair1, &pair2);

    // pair1.print();
    // pair2.print();
    // pair3.print();
}
