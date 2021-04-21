fn main() {
    // {} will do string replacement with the following arguments
    println!("{} days in a month", 31);

    // You can assign numbers to the brackets for positional arguments
    println!("{0} likes {1} and {1} likes {0}", "Alice", "Bob");

    // You can also name the objects
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // There are special formatters that can be used on objects
    println!("{0} is {0:b} in binary", 100);

    // You can right align stuff with a specific width
    println!("{number:>width$}", number=20, width=8);

    // And you can pad the extra space with a character
    println!("{number:*>width$}", number=20, width=8);
}
