extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod condition;

use std::io;

use condition::ConditionList;

fn main() {
    let stdin = io::stdin();

    if let Ok(conditions) = serde_yaml::from_reader(stdin) {
        let c = ConditionList(conditions);
        println!("<html>");
        println!("<head>");
        println!("<link rel='stylesheet' href='styles.css'>");
        println!("</head>");
        println!("<body>");
        println!("{}", c);
        println!("</body>");
        println!("</html>");
    } else {
        panic!("Error reading YAML from STDIN");
    }
}
