// References

//instead of  taking ownership of the value, it just borrows it.
// & is used as a reference.
// there are two types of references: immutable and mutable.
// immutable references are used when you don't want to modify the value.
// mutable references are used when you want to modify the value.

// There are two rules for references:
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.

// fn main() {
//     let s1 = String::from("hello");
//     let reference = &s1; // reference to s1 instead of taking ownership of s1.
//     println!("'{}',{}.", reference, s1);
// }

// fn main() {
//     let mut s = String::from("Welcome");
//     // let r1 = &s; // immutable reference
//     let r2 = &mut s; // mutable reference
//     r2.push_str(" to Rust!");
//     println!("{}", s);
// }

// fn main() {
//     let mut s = String::from("Welcome");
//     let r1 = &s; // immutable reference
//     let r2 = &s; // mutable reference
//     let r3 = &mut s; // mutable reference
//     println!("{}", r1);
//     println!("{}", r2);
//     println!("{}", r3);
//     // either one immutable reference or any number of mutable references.
// }

// fn main() {
//     let mut s = String::from("Welcome");
//     {
//         let r1 = &s; // mutable reference
//         let r2 = &s; // mutable reference
//         println!("{} and {}", r1, r2);
//     } // r1 and r2 are dropped here, but because s is borrowed, nothing happens.
//     let r3 = &mut s; // mutable reference
//     r3.push_str(" to Rust!");
//     println!("{}", s);
// }

// dangling errors
// fn main() {
//     let reference = dangle();
//     println!("{}", reference);
// }

// //  &String is a dangling pointer
// fn dangle() -> String {
//     let s = String::from("hello");
//     // &s // we are returning a reference to s but after this function ends, s will be dropped.
//     // s is dropped here, so the reference will be dangling.
//     s
// }

// fn main() {
//     let mut w = 3;
//     let x = &mut w;
//     *x += 3;
//     println!("{}", w);
// }

fn main() {
    let mut account = BankAccount {
        owner: "John".to_string(),
        balance: 1000.0,
    };

    account.balance();

    account.withdraw(11.57);

    account.balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn balance(&self) {
        println!("Balance of {} is {:.2}", self.owner, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient funds");
        } else {
            self.balance -= amount;
            println!("Withdrawn {} from {}", amount, self.owner);
        }
    }
}
