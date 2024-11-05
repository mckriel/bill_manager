use crate::billing::{Bill, BillCollection};
use crate::billing_inputs::*;

pub fn add_bill(bills: &mut BillCollection) {
    println!("Bill name: ");
    let name: String = match get_string() {
        Some(input) => input,
        None => return,
    };
    let amount: f64 = match get_amount() {
        Some(amount) => amount,
        None => return,
    };

    let bill: Bill = Bill { name, amount };
    println!("");
    println!("Bill {} added successfully!", bill.name);
    println!("");
    bills.add(bill);
}

pub fn view_bills(bills: &BillCollection) {
    println!("");
    println!("== View All Bills ==");
    for bill in bills.get_all() {
        println!("NAME: {} | AMOUNT: {}", bill.name, bill.amount);
    }
    println!("");

}

pub fn remove_bill(bills: &mut BillCollection) {
    println!("");
    println!("== Remove Bill ==");
    for bill in bills.get_all() {
        println!("NAME: {} | AMOUNT: {}", bill.name, bill.amount);
    }
    println!("");

    println!("Enter bill name to remove: ");

    let name = match get_string() {
        Some(name) => name,
        None => return
    };

    if bills.remove(&name) {
        println!("");
        println!("'{}' removed successfully", name);
    }
    else {
        println!("Bill not found");
    }
}

pub fn update_bill(bills: &mut BillCollection) {
    println!("");
    println!("== Update Bill ==");
    for bill in bills.get_all() {
        println!("NAME: {} | AMOUNT: {}", bill.name, bill.amount);
    }
    println!("");


    println!("Enter bill name to update");

    let name = match get_string() {
        Some(name) => name,
        None => return,
    };

    let amount = match get_amount() {
        Some(amount) => amount,
        None => return,
    };

    if bills.update(&name, amount) {
        println!("Bill updated!");
    }
    else {
        println!("Bill not found");
    }
    println!("");
}