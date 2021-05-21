pub fn twelve_days_christmas() {

    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let cardinal = ["a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve"];

    let gifts = ["partridge in a pear tree", "turtle doves", "French hens", "calling birds", "gold rings", "geese a laying", "swans a swimming", "maids a milking", "ladies dancing", "lords a leaping", "pipers piping", "drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love gave to me", ordinal[i]);
        
        for j in (0..i+1).rev() {
            if j != 0 {
                print!("{} {}, ", cardinal[j], gifts[j]);
            } else {
                println!("and {} {}.", cardinal[j], gifts[j]);
            }
        }

        println!();
    }
}