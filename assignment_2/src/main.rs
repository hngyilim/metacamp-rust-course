#[derive(PartialEq, Debug)]
enum PaymentType {
    DigitalToken,
    Cash,
}

struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

#[derive(Debug)]
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}

#[derive(Debug)]
struct BuyerGroup {
    members: Vec<Buyer>,
}

impl BuyerGroup {
    fn add_member(&mut self, buyer: Buyer){
        self.members.push(buyer);
    }
    fn find_buyer(&self, p_type: PaymentType) -> i32{
        for buyer_index in 0..self.members.len() {
            if p_type == self.members[buyer_index].payment_type {
                return buyer_index.try_into().unwrap();
            }
        }
        -1
    }
    fn buy(&mut self, buyer_index: i32, seller: &mut Seller){
        let buyer_balance = self.members[buyer_index as usize].balance;
        let buy_qty = (buyer_balance - (buyer_balance % seller.price)) / seller.price;
        let total_price = buy_qty * seller.price;

        seller.balance += total_price;
        self.members[buyer_index as usize].balance -= total_price;
    }
}

fn main() {
    let john_buyer = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00,
    };

    let sally_buyer = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.00,
    };

    let mut buyer_group = BuyerGroup { members: Vec::new() };
    buyer_group.add_member(john_buyer);
    buyer_group.add_member(sally_buyer);

    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.00,
        balance: 0.00,
    };

    let buyer_index = buyer_group.find_buyer(PaymentType::Cash);

    buyer_group.buy(buyer_index, &mut seller);
    println!("Buyer Group: {:?}", buyer_group);
}