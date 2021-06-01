mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    let done: Done = Done::new("shopping");
    println!("{} - {}", done.super_struct.status, done.super_struct.title);
    let pending: Pending = Pending::new("cleaning");
    println!("{} - {}", pending.super_struct.status, pending.super_struct.title);
}
