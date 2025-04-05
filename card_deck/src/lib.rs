use rand::Rng; // To generate random values

// Enum to represent the four suits in a deck of cards
#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    // Associated function to get a random Suit
    pub fn random() -> Suit {
        let mut rng = rand::rng();
        match rng.random_range(1..=4) {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(),
        }
    }

    // Associated function to convert an integer value to a Suit
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}

// Enum to represent the ranks of cards
#[derive(Debug)]
pub enum Rank {
    Ace,
    Number(u8), // For values from 2 to 10
    Jack,
    Queen,
    King,
}

impl Rank {
    // Associated function to get a random Rank
    pub fn random() -> Rank {
        let mut rng = rand::rng();
        match rng.random_range(1..=13) {
            1 => Rank::Ace,
            2..=10 => Rank::Number(rng.random_range(2..=10)),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }

    // Associated function to convert an integer value to a Rank
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
        }
    }
}

// Struct to represent a Card, containing a Suit and a Rank
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// Function to check if the card is the Ace of Spades
pub fn winner_card(card: Card) -> bool {
    matches!(card.rank, Rank::Ace) && matches!(card.suit, Suit::Spade)
}
