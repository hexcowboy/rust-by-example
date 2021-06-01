mod to_do;

use to_do::ItemTypes;
use to_do::to_do_factory;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => println!("Pending: {}", item.super_struct.title),
        ItemTypes::Done(item) => println!("Done: {}", item.super_struct.title)
    }
}
