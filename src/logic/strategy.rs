use rocket::serde::json::Json;

use crate::models::player::Player;

pub fn decide(_table: Json<crate::models::table::Table>) -> crate::models::bet::Bet {
    println!("Table: {:?}", _table);

    // vector number of size 13
    let mut number: Vec<i32> = vec![0; 13];

    // vector straight of size 13 of booleans
    let mut straight: Vec<bool> = vec![false; 13];
    
    // vector color of size 4 of pair of Suit and integer (all 4 suits)
    let mut color: Vec<(crate::models::suit::Suit, i32)> = vec![(crate::models::suit::Suit::HEARTS, 0), (crate::models::suit::Suit::SPADES, 0), (crate::models::suit::Suit::CLUBS, 0), (crate::models::suit::Suit::DIAMONDS, 0)];

    // number of hidden community cards
    let mut hidden_community_cards: i32 = 5 - _table.community_cards.len() as i32;

    // get cards for player "Texas Hold'team" and count the rank in vector number
    for _player in &_table.players {
        if _player.name == "Texas Hold'team" {
            for _card_vec in &_player.cards {
                for _card in _card_vec {
                    match _card.rank {
                        crate::models::rank::Rank::A => number[0] += 1,
                        crate::models::rank::Rank::K => number[1] += 1,
                        crate::models::rank::Rank::Q => number[2] += 1,
                        crate::models::rank::Rank::J => number[3] += 1,
                        crate::models::rank::Rank::_10 => number[4] += 1,
                        crate::models::rank::Rank::_9 => number[5] += 1,
                        crate::models::rank::Rank::_8 => number[6] += 1,
                        crate::models::rank::Rank::_7 => number[7] += 1,
                        crate::models::rank::Rank::_6 => number[8] += 1,
                        crate::models::rank::Rank::_5 => number[9] += 1,
                        crate::models::rank::Rank::_4 => number[10] += 1,
                        crate::models::rank::Rank::_3 => number[11] += 1,
                        crate::models::rank::Rank::_2 => number[12] += 1,
                    }
                }
            }

            // fill vector strait with true if the corresponding card exists in vector number


            // fill vector color with the number of cards of the corresponding suit
            for _card_vec in &_player.cards {
                for _card in _card_vec {
                    match _card.suit {
                        crate::models::suit::Suit::HEARTS => color[0].1 += 1,
                        crate::models::suit::Suit::SPADES => color[1].1 += 1,
                        crate::models::suit::Suit::CLUBS => color[2].1 += 1,
                        crate::models::suit::Suit::DIAMONDS => color[3].1 += 1,
                    }
                }
            }
        }
    } 

    let tab_commie_cards_1 = _table.community_cards.clone();
    let tab_commie_cards_2 = _table.community_cards.clone();
    
        for _card in tab_commie_cards_1 {
            match _card.rank {
                crate::models::rank::Rank::A => number[0] += 1,
                crate::models::rank::Rank::K => number[1] += 1,
                crate::models::rank::Rank::Q => number[2] += 1,
                crate::models::rank::Rank::J => number[3] += 1,
                crate::models::rank::Rank::_10 => number[4] += 1,
                crate::models::rank::Rank::_9 => number[5] += 1,
                crate::models::rank::Rank::_8 => number[6] += 1,
                crate::models::rank::Rank::_7 => number[7] += 1,
                crate::models::rank::Rank::_6 => number[8] += 1,
                crate::models::rank::Rank::_5 => number[9] += 1,
                crate::models::rank::Rank::_4 => number[10] += 1,
                crate::models::rank::Rank::_3 => number[11] += 1,
                crate::models::rank::Rank::_2 => number[12] += 1,
            }
        }

    // fill vector strait with true if the corresponding card exists in vector number
    for i in 0..13 {
        if number[i] > 0  {
            straight[i] = true;
        }
    }

    // fill vector color with the number of cards of the corresponding suit
        for _card in tab_commie_cards_2 {
            match _card.suit {
                crate::models::suit::Suit::HEARTS => color[0].1 += 1,
                crate::models::suit::Suit::SPADES => color[1].1 += 1,
                crate::models::suit::Suit::CLUBS => color[2].1 += 1,
                crate::models::suit::Suit::DIAMONDS => color[3].1 += 1,
            }
        }

    // find card with highest value in vector number, begin from the end of the vector
    let mut highest_card_value: i32 = 0;
    let mut highest_card: i32 = 0;
    let mut temp = 0;

    for i in 0..13 {
        if number[i] > temp {
            temp = number[i];
        }
    }

    for i in (0..13).rev() {
        if number[i] == temp {
            temp = number[i];
            highest_card = i as i32;
            highest_card_value = temp;
            break
        }
    }

    let mut highest_card_count: i32 = 0;
    
    // p = 4 - number[highest_card] / (52 - (7 - hidden_community_cards))
    let mut p: f32 = 4.0 - number[highest_card as usize] as f32 / (52.0 - (7.0 - hidden_community_cards as f32));

    // check all windows of 5 cards in vector straight and find the window with the fewest false values
    let mut min_missing_cards: i32 = 0;
    let mut min_missing_cards_count: i32 = 0;
    for i in 0..9 {
        let mut min_missing_cards_temp: i32 = 0;
        for j in i..i+5 {
            if !straight[j] {
                min_missing_cards_temp += 1;
            }
        }

        if min_missing_cards_temp == min_missing_cards {
            min_missing_cards_count += 1;
        }

        if min_missing_cards_temp < min_missing_cards {
            min_missing_cards = min_missing_cards_temp;
            min_missing_cards_count = 1;
        }
    }

    let mut p_prime = 0;
    if min_missing_cards > hidden_community_cards{
            p_prime = 0;
       } else {
            for i in 0..min_missing_cards {
               p_prime = (4 * (min_missing_cards - i)) / (52 - (7 - hidden_community_cards));
            }
        };

    let color_lul = color.clone();
    let color_lel = color.clone();  

    p_prime = p_prime * min_missing_cards_count;
// find maximum number of cards in color
    let mut max_color: i32 = 0;
    for c   in color_lul {
        if c.1 > max_color {
            max_color = c.1;
        }
    }

    let mut max_color_count: i32 = 0;
    for c in color_lel {
           if c.1 == max_color {
               max_color_count += 1;
          }
        }
    
    let flush: bool = max_color >= 5;
    let flush_viable = hidden_community_cards + (5 - max_color) >= 5;

    let mut p_prime_prime = 0;
    if flush_viable && !flush {
        let mut temp = 1;
        for i in 0..(5 - max_color) {
            temp = temp * ((13 - max_color - i) / (52 - (7 - hidden_community_cards) -i ));
         };

        p_prime_prime = temp * max_color_count;
    };

    let mut Three_ofs : i32 = 0;
    let mut pairs : i32 = 0;
    let mut three_of_pos = Vec::new();
    let mut pair_pos = Vec::new();
    let mut full_house = false;

    for n in number {
        if n >= 3 {
            three_of_pos.push(n);
        };
        if n >= 2 && !three_of_pos.contains(&n){
            pair_pos.push(n);
            pairs += 1;
        };
    }

    if three_of_pos.len() >= 2 || (three_of_pos.len() >= 1 && pair_pos.len() >= 1){
        full_house = true;
    };

    let mut two_pair = pairs >= 2;

    let full_house = Three_ofs + pairs >= 2 && !pairs >=2;

   let straight = min_missing_cards == 0;
    // highest card is the highest card in the player's hand

    let all_in = (straight && flush) || (highest_card_count == 4);
    let moderatly_in = full_house || flush || straight;

    
let binding = _table.players.clone().into_iter().filter(|player| player.name == "Texas Hold'team").collect::<Vec<Player>>();
let us1 = binding.first();
let us = us1.unwrap();
let mut alpha:f32 = 0.0;

if highest_card_count == 3 {
    alpha = 8.0;
} else if pairs >= 2 {
    alpha = 7.0;
} else if highest_card_count == 2 {
    alpha = 5.0;
} else 
{   alpha = 2.0;}

let mut bet = 0;
 if all_in {
    bet = us.stack;
    return crate::models::bet::Bet{bet: bet};
    }  else if moderatly_in {
    bet = us.stack / 2;
    return crate::models::bet::Bet{bet: bet};
   } else {
     bet = (p * highest_card_value as f32 * alpha as f32
                  + p_prime as f32 * 100.0
                  + p_prime_prime as f32 * 100.0) as i32;
     }
    

return crate::models::bet::Bet{bet: bet};

    }