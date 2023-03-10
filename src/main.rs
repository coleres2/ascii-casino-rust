
// use shuffle::shuffler::Shuffler;
// use shuffle::irs::Irs;
//use rand::rngs::mock::StepRng;
use crate::rand::prelude::SliceRandom;
use std::sync::atomic::{AtomicUsize, Ordering};

extern crate shuffle;
extern crate rand;

#[derive(Clone, Default, Debug)]
struct Card {
    suit: String,
    value: String,
}
trait CardTrait {
    fn new(suit: String, value: String) -> Card;
    fn get_suit(&self) -> String;
    fn get_value(&self) -> String;
    fn get_blackjack_value(&self) -> u32;
}

impl CardTrait for Card {
    fn new(suit: String, value: String) -> Card { //Creates a new card. Might not be used much
        Card {
            suit: suit,
            value: value,
        }
    }
    fn get_suit(&self) -> String {
        self.suit.clone()
    }
    fn get_value(&self) -> String { 
        self.value.clone()
    }
    fn get_blackjack_value(&self) -> u32 { //May have player score as a parameter to decide whether to use 1 or 11. Can also do in blackjac function.
        //let my_int: i32 = self.value.parse().unwrap();
        if self.value.eq("A") {
            return 11;
        }
        if self.value.eq("J") || self.value.eq("Q") || self.value.eq("K") {
            return 10;
        }
        return self.value.parse().unwrap();
        //self.blackjack_value
    }
}
#[derive(Clone)]
struct Deck {
    cards: Vec<Card>,
}

trait DeckTrait {
    fn new() -> Deck;
    fn get_cards(&self) -> Vec<Card>;
    fn shuffle_deck(&mut self);
    fn deal(&mut self) -> Card;
}

impl DeckTrait for Deck {
    fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        let suits = vec!["Hearts", "Diamonds", "Spades", "Clubs"];
        let values = vec![
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];
        for suit in suits {
            for value in &values {
                cards.push(Card::new(suit.to_string(), value.to_string()));
            }
        }
        Deck { cards: cards }
    }
    fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }
    fn shuffle_deck(&mut self) {
        //let rng = StepRng::new(0, 1);
        //let mut irs = Irs::default();
        //irs.shuffle(&mut self.cards, &mut rng);
        self.cards.shuffle(&mut rand::thread_rng());
    }
    fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}
#[derive(Clone)]
struct Player {
    name: String,
    hand: Vec<Card>,
    money: i64,
}

trait PlayerTrait {
    fn new() -> Player;
    fn get_name(&self) -> String;
    fn get_hand(&self) -> Vec<Card>;
}

impl PlayerTrait for Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            hand: Vec::new(),
            money: 0,
        }
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }
}
#[derive(Clone)]
struct Table {
    number: u32,
    players: Vec<Player>,
    dealer: Player,
    deck: Deck,
    max_bet: u32,   
}
static mut TABLE_COUNT: u32 = 0;
trait TableTrait {
    fn new() -> Table;
    fn add_player(self);
    fn remove_player();
    fn get_number(&self) -> u32;
}

impl TableTrait for Table {
    fn new() -> Table {
        unsafe { //I know this isn't the best way. Will come back to later
            let table_count = TABLE_COUNT;
            TABLE_COUNT += 1;
            Table {
                number: table_count,
                players: Vec::new(),
                dealer: Player::new(),
                deck: Deck::new(),
                max_bet: 200,   
            }
        }
    }
    fn add_player(mut self) {
        let player = Player::new();
        self.players.push(player);
    }
    fn remove_player() {

    }
    fn get_number(&self) -> u32 {
        self.number.clone()
    }
}
#[derive(Clone)]
struct Blackjack {
    table: Table,
}

trait BlackjackTrait {
    fn new() -> Blackjack;
    fn start_game(&self);
}

impl BlackjackTrait for Blackjack {
    fn new() -> Blackjack {
        Blackjack { table: Table::new() }
    }
    fn start_game(&self) {
        // for player in self.table.players {
            
        // }
    }
}

fn main() {
    let card = Card::new("Hearts".to_string(), "A".to_string());
    println!("{} of {}", card.get_value(), card.get_suit());
    println!("Blackjack value: {}", card.get_blackjack_value());
    let mut deck = Deck::new();
    println!("Deck size: {}", deck.get_cards().len());
    deck.shuffle_deck();
    println!("Deck size: {}", deck.get_cards().len());
    let card = deck.deal();
    println!("{} of {}", card.get_value(), card.get_suit());
    println!("Deck size: {}", deck.get_cards().len());
    let mut player = Player::new();
    player.hand.push(card);
    println!("Player hand: {:?}", player.get_hand());

}