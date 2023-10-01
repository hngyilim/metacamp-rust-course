struct User {
    name: String,
    balance: (f32, String)
}

impl User {
    fn print_user_detail(&self) {
        println!("Name: {}, Balance: {:?}",self.name, self.balance)
    }
}

fn main() {
    let mut user = User {
    name: "John".to_owned(),
    balance: (100.0, "SGD".to_owned()),
    };

    for _ in 1..=5 {
        accrue_interest(&mut user, 10.0);
    }
}

fn accrue_interest( user: &mut User, interest: f32) {
    user.balance.0 = user.balance.0 + (user.balance.0 * interest / 100.0);
    user.print_user_detail();
}