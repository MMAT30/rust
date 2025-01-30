fn main() {
    let card = Card {
        suit: CardSuit::Hearts,
        rank: CardRank::Ace,
    };

    let deck: Cards = Cards {
        cards: vec![
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Ace,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Two,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Three,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Four,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Five,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Six,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Seven,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Eight,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Nine,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Ten,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Jack,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Queen,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::King,
            },
            Card {
                suit: CardSuit::Hearts,
                rank: CardRank::Ace,
            },
            Card {
                suit: CardSuit::Diamonds,
                rank: CardRank::Two,
            },
            Card {
                suit: CardSuit::Spades,
                rank: CardRank::Three,
            },
            Card {
                suit: CardSuit::Clubs,
                rank: CardRank::Four,
            },
        ],
    };

    print!("Deck: {:?}\n", deck.cards);
    println!("Card: {:?} {:?}\n", card.rank, card.suit);


    // payment method
    let payment1 = PaymentMethod::Cash(100.0);
    let payment2 = PaymentMethod::CreditCard {
        name: String::from("John Doe"),
        card_number: 1234567890,
        cvv: 123,
    };

    payment1.print_payment();
    payment2.print_payment();

    // if let statement
    let payment3 = PaymentMethod::Cash(12.50);
    if let PaymentMethod::Cash(12.50) = payment3 {
        println!("Paying with cash: ${}", 12.50);
    } else {
        println!("Paying with cash: ${:?}", payment3);
    }


    // option
    let arr = [1, 2, 3, 4, 5];
    let v1: Option<&i32> = arr.get(1);
    let v2: Option<&i32> = arr.get(10);


    match v1 {
        Some(value) => println!("v1: {}", value),
        None => println!("v1: None"),
    }

    match v2 {
        Some(value) => println!("v2: {}", value),
        None => println!("v2: None"),
    }


    // result
    let res1: Result<i32, &str> = Ok(200);
    let res2: Result<i32, &str> = Err("Error: 404");

    match res1 {
        Ok(value) => println!("res1: {}", value),
        Err(value) => println!("res1: {}", value),
    }

    match res2 {
        Ok(value) => println!("res2: {}", value),
        Err(value) => println!("res2: {}", value),
    }



    // unwrap - expect
    let v3 = v1.unwrap();
    println!("v3: {}", v3);

    let v4 = v2.expect("Index out of bounds");
    println!("v4: {}", v4);
}

#[derive(Debug)]
struct Card {
    suit: CardSuit,
    rank: CardRank,
}

#[derive(Debug)]
struct Cards {
    cards: Vec<Card>,
}

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
enum CardRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
enum PaymentMethod {
    Cash(f64),
    CreditCard {
        name: String,
        card_number: i64,
        cvv: i8,
    },
}


impl PaymentMethod {
    fn print_payment(&self) {
        match self {
            PaymentMethod::Cash(12.50) => {
                println!("Paying with cash: $12.50");
            }
            PaymentMethod::Cash(amount) => {
                println!("Paying with cash: ${}", amount);
            }
            PaymentMethod::CreditCard {
                name,
                card_number,
                cvv,
            } => {
                println!("Paying with credit card: {} {} {}", name, card_number, cvv);
            }
        }
    }
}
