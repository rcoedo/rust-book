const DAY_NUMBER: [&str; 12] = [
    "first", "second", "thrid", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth",
    "eleventh", "twelfth",
];
const STUFF_MY_LOVER_SENT: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle-doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings (five golden rings)",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    for i in 0..12 {
        println!("On the {} day of Christmas", DAY_NUMBER[i]);
        println!("My true love sent to me");
        for j in (0..i + 1).rev() {
            println!(
                "{}",
                if i > 0 && j == 0 {
                    format!("And {}", STUFF_MY_LOVER_SENT[j].to_lowercase())
                } else {
                    STUFF_MY_LOVER_SENT[j].to_owned()
                }
            );
        }
        println!("")
    }
}
