

fn main() {
    let gifts = vec![
        "a partridge in a pear tree!",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let ordinal = vec![
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let mut previous_lines: Vec<String> = vec![];

    for i in 1..13 {
        println!("On the {} day of Christmas my true love gave to me...", ordinal[i -1]);
    let formatted_line = format!("{}", gifts[i -1]);     
        println!("{}", formatted_line);
        println!("{}", previous_lines.join(" "));
        previous_lines.insert(0, formatted_line);

    }
}
