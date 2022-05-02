fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fith", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelth",
    ];

    const GIFTS: [&str; 12] = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    // Unroll the first element in the loop since it has slight different
    // wording the first time.
    println!("On the first day of Christmas my true love sent to me:");
    println!("A partridge in a a pear tree.");

    for day in 1..12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            DAYS[day]
        );
        for gift in (0..day + 1).rev() {
            println!("{}", GIFTS[gift]);
        }
    }
}
