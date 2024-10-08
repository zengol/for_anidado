

#[derive(Debug)]

struct Desk {
    cards : Vec<String>,
}

fn main() {

    let suits = ["Hearts","Spades","Diamonds"];

    let values= ["one", "two", "three"];

    let mut cards = vec![];

    for suit in suits {

        for value in values{

            let card = format!("{} of {} ", value, suit);
            
            cards.push(card);
        }
    }
    //inicializacion de struct desk

    let desk = Desk { cards };

    println!("Your cards are: {:#?}", desk);
}
