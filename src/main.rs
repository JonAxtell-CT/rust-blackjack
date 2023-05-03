//%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// Rust implementation of a simple blackjack simulator. Based on C++ code
// written by Dave Plummer
// See https://github.com/davepl/blackjack/blob/main/blackjack.cpp
//
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::slice::Iter;

//#############################################################################
// Card's rank (numeric value)
//
#[derive(Copy, Clone, Debug, PartialEq)]
enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl Rank {
    /// Return the numeric value of the card's rank. Ace is 1 and
    /// Jack is 11, Queen is 12, and King is 13.
    fn as_number(&self) -> u8 {
        match self {
            Rank::ACE => 1,
            Rank::TWO => 2,
            Rank::THREE => 3,
            Rank::FOUR => 4,
            Rank::FIVE => 5,
            Rank::SIX => 6,
            Rank::SEVEN => 7,
            Rank::EIGHT => 8,
            Rank::NINE => 9,
            Rank::TEN => 10,
            Rank::JACK => 11,
            Rank::QUEEN => 12,
            Rank::KING => 13,
        }
    }

    /// Return the rank of the card as a string. For display purposes.
    fn as_string(&self) -> String {
        match self {
            Rank::ACE => "ACE".to_string(),
            Rank::TWO => "TWO".to_string(),
            Rank::THREE => "THREE".to_string(),
            Rank::FOUR => "FOUR".to_string(),
            Rank::FIVE => "FIVE".to_string(),
            Rank::SIX => "SIX".to_string(),
            Rank::SEVEN => "SEVEN".to_string(),
            Rank::EIGHT => "EIGHT".to_string(),
            Rank::NINE => "NINE".to_string(),
            Rank::TEN => "TEN".to_string(),
            Rank::JACK => "JACK".to_string(),
            Rank::QUEEN => "QUEEN".to_string(),
            Rank::KING => "KING".to_string(),
        }
    }

    /// Return the rank of the card as a string. For display purposes.
    fn as_character(&self) -> String {
        match self {
            Rank::ACE => "A".to_string(),
            Rank::TWO => "2".to_string(),
            Rank::THREE => "3".to_string(),
            Rank::FOUR => "4".to_string(),
            Rank::FIVE => "5".to_string(),
            Rank::SIX => "6".to_string(),
            Rank::SEVEN => "7".to_string(),
            Rank::EIGHT => "8".to_string(),
            Rank::NINE => "9".to_string(),
            Rank::TEN => "0".to_string(),
            Rank::JACK => "J".to_string(),
            Rank::QUEEN => "Q".to_string(),
            Rank::KING => "K".to_string(),
        }
    }

    /// Allow rank to be used in an iterator. Iteration will progress from ACE to KING.
    /// Note: Trick is to have a static array and iterate through that.
    fn iterator() -> Iter<'static, Rank> {
        static RANK: [Rank; 13] = [
            Rank::ACE,
            Rank::TWO,
            Rank::THREE,
            Rank::FOUR,
            Rank::FIVE,
            Rank::SIX,
            Rank::SEVEN,
            Rank::EIGHT,
            Rank::NINE,
            Rank::TEN,
            Rank::JACK,
            Rank::QUEEN,
            Rank::KING,
        ];
        RANK.iter()
    }
}

//#############################################################################
// Card's suit
//
#[derive(Copy, Clone, Debug, PartialEq)]
enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

impl Suit {

    /// Return the suit of the card as a string. For display purposes.
    fn as_string(&self) -> String {
        match self {
            Suit::HEARTS => "HEARTS".to_string(),
            Suit::DIAMONDS => "DIAMONDS".to_string(),
            Suit::CLUBS => "CLUBS".to_string(),
            Suit::SPADES => "SPADES".to_string(),
        }
    }

    /// Return the suit of the card as a string. For display purposes.
    fn as_character(&self) -> String {
        match self {
            Suit::HEARTS => "♥".to_string(),
            Suit::DIAMONDS => "♦".to_string(),
            Suit::CLUBS => "♣".to_string(),
            Suit::SPADES => "♠".to_string(),
        }
    }

    /// Allow suit to be used in an iterator. Iteration will progress from HEARTS to SPADES.
    /// Note: Trick is to have a static array and iterate through that.
    fn iterator() -> Iter<'static, Suit> {
        static SUIT: [Suit; 4] = [Suit::HEARTS, Suit::DIAMONDS, Suit::CLUBS, Suit::SPADES];
        SUIT.iter()
    }
}

//#############################################################################
// A single playing card
//
#[derive(Copy, Clone, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank.as_character(), self.suit.as_character())
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank.as_string(), self.suit.as_string())
    }
}

//#############################################################################
// A bunch of playing cards
//
// A tuple struct to implement manipulation of a vec of Card. Deref to allow
// iteration, Display for outputting and new() for creation.
//
// Note: See https://github.com/apolitical/impl-display-for-vec
#[derive(Debug, PartialEq)]
struct Cards(pub Vec<Card>);

// Allows code like the following to be used:
//  ```deck.cards.iter().for_each(|card| println!("{}", card));```
impl std::ops::Deref for Cards {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Cards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Allows code like the following to be used:
//  ```println!("{}", Cards(deck.cards().to_vec()));```
impl fmt::Display for Cards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| write!(f, "{}, ", card))
        })
    }
}

impl Cards {
    fn new() -> Self {
        Cards(Vec::<Card>::new())
    }
}

//#############################################################################
// The deck of cards that are used to deal to the players from
//
#[derive(Debug)]
struct Deck {
    cards: Cards,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Cards::new();
        for s in Suit::iterator() {
            for r in Rank::iterator() {
                cards.push(Card::new(*r, *s));
            }
        }
        Self { cards }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn number_of_cards(&self) -> u8 {
        // Note: try_into().unwrap() is converting usize into u8
        self.cards.len().try_into().unwrap()
    }
}

//#############################################################################
// A player (or the dealer) who holds a hand of cards
//
#[derive(Debug)]
struct Player {
    hand: Cards,
}

impl Player {
    fn new() -> Self {
        let hand = Cards::new();
        Self { hand }
    }

    fn add_card(self: &mut Player, card: Option<Card>) {
        self.hand.push(card.unwrap())
    }

    fn get_hand_value(self: &mut Player) -> u8 {
        let mut value = 0;
        let mut aces = 0;

        self.hand.iter().for_each(|card| {
            let mut card_value = card.rank.as_number();
            if card_value >= 10 {
                card_value = 10;
            } else if card_value == 1 {
                aces += 1;
                card_value = 11;
            }
            value += card_value;

        });

        while value > 21 && aces > 0 {
            value -= 10;
            aces -= 1;
        }
        value
    }
}

//#############################################################################
//
fn main() {
    // Create the players
    let mut player = Player::new();
    let mut dealer = Player::new();

    // Create the deck and shuffle it
    let mut deck = Deck::new();
    deck.shuffle();

    // Deal out the hands
    player.add_card(deck.draw_card());
    player.add_card(deck.draw_card());

    dealer.add_card(deck.draw_card());
    dealer.add_card(deck.draw_card());

    // Show the hands
    print!("Dealer hand: ");
    dealer.hand.iter().for_each(|card| print!("{}, ", card));
    println!("value: {}", dealer.get_hand_value());

    print!("Player hand: ");
    player.hand.iter().for_each(|card| print!("{}, ", card));
    println!("value: {}", player.get_hand_value());

    // Who has won?
    if dealer.get_hand_value() > player.get_hand_value() {
        println!("Dealer wins. Boo!");
    }
    else {
        println!("Player wins. Yae!");
    }

    // Output whole pack using fmt::Display for Cards
    // Note: Requires to_vec() since can't copy a vec for Cards so a copy needs to be made
    println!(
        "What's left in the deck of {} cards",
        deck.number_of_cards()
    );
    println!("{}", Cards(deck.cards.to_vec()));
    println!("{:?}", Cards(deck.cards.to_vec()));
}

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_creation() {
        let card = Card::new(Rank::ACE, Suit::DIAMONDS);
        assert_eq!(card.suit.as_string(), "DIAMONDS");
        assert_eq!(card.rank.as_string(), "ACE");
    }

    #[test]
    fn deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.number_of_cards(), 52);
    }

    #[test]
    fn deck_has_been_shuffled() {
        let mut deck = Deck::new();
        assert_eq!(deck.cards[0], Card::new(Rank::ACE, Suit::HEARTS));
        deck.shuffle();
        // Note: Could fail and the top of the deck could still be ace of hearts
        // after shuffling, maybe test a few more cards?
        assert_ne!(deck.cards[0], Card::new(Rank::ACE, Suit::HEARTS));
    }

    #[test]
    fn drawing_card_removes_from_deck() {
        let mut deck = Deck::new();
        deck.draw_card();
        assert_eq!(deck.number_of_cards(), 51);
    }

    #[test]
    fn adding_card_to_players_hand() {
        let mut player = Player::new();
        let card = Card::new(Rank::ACE, Suit::DIAMONDS);
        player.add_card(std::option::Option::Some(card));
        assert_eq!(player.get_hand_value(), 11);
    }
}
