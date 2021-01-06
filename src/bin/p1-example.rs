// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::collections::HashMap;
use std::io;

/// A bill with a name and amount owed.
#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

/// Collection used to store bills.
struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    /// Create a new bills collection.
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Add a new bill. If a bill with the same name exists, it is overwritten.
    fn add(&mut self, bill: Bill) {
        // We need to clone the bill name, since the String type cannot be implicitly copied.
        // Without the clone, the name would get moved into the 'key' portion of the hashmap,
        // and therefore would be moved out of the bill struct.
        self.inner.insert(bill.name.clone(), bill);
    }

    /// Retrieve all the bills.
    fn get_all(&self) -> Vec<&Bill> {
        let mut bills = vec![];
        // Iterate through each value of the bill hashmap, ignoring the keys.
        for bill in self.inner.values() {
            // Slight change made after the video was created: We are using
            // a borrow here to make the program more efficient. When iterating
            // using .values(), the value is borrowed automatically.
            bills.push(bill);
        }
        bills
    }

    /// Removes an existing bill. Returns false if the bill does not exist.
    fn remove(&mut self, name: &str) -> bool {
        // Chaning the is_some() function call will allow us to return
        // whether an item was removed or not.
        self.inner.remove(name).is_some()
    }

    /// Updates an existing bill. Returns false if the bill does not exist.
    fn update(&mut self, name: &str, amount: f64) -> bool {
        // We use the get_mut() function defined on the HashMap type
        // in order to change items present within the hashmap.
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

/// Retrieves user input. This function will automatically retry on
/// io errors, and will return None if the user did not enter any data.
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

/// Retrieves a bill amount. None is returned if the user did not
/// make any entry, otherwise will retry until the user enters an amount.
fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

/// Process for adding a new bill. Includes accepting user input
/// and aborting if the user does not enter any data.
fn add_bill_menu(bills: &mut Bills) {
    println!("Bill name:");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    let bill = Bill { name, amount };
    bills.add(bill);
    println!("Bill added");
}

/// Process for removing an existing bill. Includes accepting user
/// input and aborting if the user does not enter any data.
fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to remove:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    if bills.remove(&name) {
        println!("removed");
    } else {
        println!("bill not found");
    }
}

/// Process for updating an existing bill. Includes accepting user
/// input and aborting if the user does not enter any data.
fn update_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill to update:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    if bills.update(&name, amount) {
        println!("updated");
    } else {
        println!("bill not found");
    }
}

/// Process for viewing existing bills.
fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

/// Main menu loop.
///
/// Displays the main menu and allows the user to make a selection.
/// Any entry that does not exist will abort the program.
fn main_menu() {
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("");
        println!("Enter selection:");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => update_bill_menu(&mut bills),
            _ => break,
        }
    }
}

fn main() {
    main_menu();
}
