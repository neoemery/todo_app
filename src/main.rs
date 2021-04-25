mod to_do;

use to_do::structs::done::Done;
use to_do::structs::pending::Pending;
use to_do::ItemTypes;
use to_do::to_do_factory;


fn main() {

    let to_do_item: Result<ItemTypes, &'static str> =
        to_do_factory("pending", "make");
    
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => println!(
            "it's a pending item with the title: {}",
            item.super_struct.title),
        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title)
    }    

    let done: Done = Done::new("shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending: Pending = Pending::new("laundry");
    println!("{}", pending.super_struct.title);    
    println!("{}", pending.super_struct.status);

}
