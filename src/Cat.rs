use std::io;

pub fn what() {
    let mut idiot = String::from("idiot");
    println!("{}", idiot); // runs first
    let _share_the_stupidity = anotherretard(idiot); // giving a borrow causes it to run the next func
}

fn anotherretard(retard: String) {
    println!("{} + retard", retard)
}