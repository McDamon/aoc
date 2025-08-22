// https://adventofcode.com/2023/day/7

use crate::utils::get_lines;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    pub bid: u64,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut sorted_cards = self.cards.to_vec();
        sorted_cards.sort();

        let groups = sorted_cards.into_iter().chunk_by(Clone::clone);

        let mut hand_type: HandType = HandType::HighCard;

        let mut five_of_a_kind_count = 0;
        let mut four_of_a_kind_count = 0;
        let mut three_of_a_kind_count = 0;
        let mut pair_count = 0;
        let mut joker_count = 0;
        for (card, group) in &groups {
            let count = group.count();
            match card {
                Card::Joker => joker_count = count,
                _ => match count {
                    5 => {
                        five_of_a_kind_count += 1;
                    }
                    4 => {
                        four_of_a_kind_count += 1;
                    }
                    3 => {
                        three_of_a_kind_count += 1;
                    }
                    2 => {
                        pair_count += 1;
                    }
                    _ => (),
                },
            }
        }

        if four_of_a_kind_count == 1 && joker_count == 1 {
            five_of_a_kind_count = 1;
            four_of_a_kind_count = 0;
        } else if three_of_a_kind_count == 1 && joker_count == 2 {
            five_of_a_kind_count = 1;
            three_of_a_kind_count = 0;
        } else if three_of_a_kind_count == 1 && joker_count == 1 {
            four_of_a_kind_count = 1;
            three_of_a_kind_count = 0;
        } else if pair_count == 2 && joker_count == 1 {
            three_of_a_kind_count = 1;
            pair_count = 1;
        } else if pair_count == 1 && joker_count == 3 {
            five_of_a_kind_count = 1;
            pair_count = 0;
        } else if pair_count == 1 && joker_count == 2 {
            four_of_a_kind_count = 1;
            pair_count = 0;
        } else if pair_count == 1 && joker_count == 1 {
            three_of_a_kind_count = 1;
            pair_count = 0;
        } else if joker_count == 4 || joker_count == 5 {
            five_of_a_kind_count = 1;
        } else if joker_count == 3 {
            four_of_a_kind_count = 1;
        } else if joker_count == 2 {
            three_of_a_kind_count = 1;
        } else if joker_count == 1 {
            pair_count = 1;
        }

        if five_of_a_kind_count == 1 {
            hand_type = HandType::FiveOfAKind;
        } else if four_of_a_kind_count == 1 {
            hand_type = HandType::FourOfAKind;
        } else if three_of_a_kind_count == 1 && pair_count == 1 {
            hand_type = HandType::FullHouse;
        } else if three_of_a_kind_count == 1 && pair_count == 0 {
            hand_type = HandType::ThreeOfAKind;
        } else if three_of_a_kind_count == 0 && pair_count == 2 {
            hand_type = HandType::TwoPair;
        } else if three_of_a_kind_count == 0 && pair_count == 1 {
            hand_type = HandType::OnePair;
        }

        hand_type
    }

    fn hand_rank(&self, other_cards: &Vec<Card>) -> Ordering {
        let scores = self
            .cards
            .iter()
            .zip(other_cards)
            .map(|(self_card, other_card)| self_card.cmp(other_card));

        for score in scores {
            match score {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => (),
            }
        }

        Ordering::Equal
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_order = self.hand_type().cmp(&other.hand_type());
        if hand_type_order == Ordering::Equal {
            self.hand_rank(&other.cards)
        } else {
            hand_type_order
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

#[derive(Debug, Default)]
pub struct Input {
    hands: Vec<Hand>,
}

pub fn parse_input(input_file: &str, use_joker: bool) -> Input {
    let lines = get_lines(input_file);

    let mut input = Input::default();

    for line in lines {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let hand = Hand {
            cards: parse_hand(line_parts.first().unwrap(), use_joker),
            bid: line_parts.last().unwrap().parse().unwrap(),
        };
        input.hands.push(hand);
    }

    input
}

fn parse_hand(hand_str: &str, use_joker: bool) -> Vec<Card> {
    let mut hand: Vec<Card> = vec![];
    for card in hand_str.chars() {
        hand.push(parse_card(card, use_joker));
    }
    hand
}

fn parse_card(card_char: char, use_joker: bool) -> Card {
    match card_char {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => {
            if use_joker {
                Card::Joker
            } else {
                Card::Jack
            }
        }
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid card: {}", card_char),
    }
}

pub fn get_total_winnings(input_file: &str, use_joker: bool) -> u64 {
    let mut total_winnings: u64 = 0;
    let mut input = parse_input(input_file, use_joker);
    input.hands.sort();
    for (rank, hand) in input.hands.iter().enumerate() {
        total_winnings += hand.bid * (rank as u64 + 1);
    }
    total_winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_winnings_test01() {
        assert_eq!(
            6440,
            get_total_winnings("input/2023/day07_test01.txt", false)
        );
    }

    #[test]
    fn test_get_total_winnings_test02() {
        assert_eq!(8, get_total_winnings("input/2023/day07_test02.txt", false));
    }

    #[test]
    fn test_get_total_winnings_test03() {
        assert_eq!(8, get_total_winnings("input/2023/day07_test03.txt", false));
    }

    #[test]
    fn test_get_total_winnings() {
        assert_eq!(251545216, get_total_winnings("input/2023/day07.txt", false));
    }

    #[test]
    fn test_get_total_winnings_joker_test01() {
        assert_eq!(
            5905,
            get_total_winnings("input/2023/day07_test01.txt", true)
        );
    }

    #[test]
    fn test_get_total_winnings_joker_test04() {
        assert_eq!(65, get_total_winnings("input/2023/day07_test04.txt", true));
    }

    #[test]
    fn test_get_total_winnings_joker() {
        assert_eq!(250384185, get_total_winnings("input/2023/day07.txt", true));
    }
}
