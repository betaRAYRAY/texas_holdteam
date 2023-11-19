#![allow(non_snake_case)]

use rocket::serde::json::Json;
use rand::prelude::*;
use strum::IntoEnumIterator;
use std::time::{Duration, Instant};
use std::cmp::{min, max};

use crate::models::player::{Player, PlayerStatusEnum};
use crate::models::table::{Table, self};
use crate::models::bet::Bet;
use crate::models::card::Card;
use crate::models::rank::Rank;
use crate::models::suit::Suit;

pub fn decide(_table: Json<Table>) -> Bet {
    let activePlayerCount: i32 = _table.players.iter().filter(|player| player.status == PlayerStatusEnum::ACTIVE).count() as i32;
    let us: &Player = &_table.players[_table.active_player as usize];
    let hand: &Vec<Card> = &us.cards.as_ref().unwrap();
    let communityCards: &Vec<Card> = &_table.community_cards;

    let simulationStart: Instant = Instant::now();
    let winProbability: f64 = simulateWinProbability(hand, communityCards, activePlayerCount - 1);
    let simulationTime: Duration = simulationStart.elapsed();

    let maxOpponentStack: i32 = _table.players.iter().filter(|player| player.status == PlayerStatusEnum::ACTIVE).map(|player| player.stack).max().unwrap_or(0);

    let bet;
    let betType: char;
    if winProbability > 1.5 / (activePlayerCount as f64) {
        bet = max(_table.minimum_raise, maxOpponentStack);
        betType = 'R';
    } else if winProbability > 1.3 / (activePlayerCount as f64) {
        bet = _table.minimum_raise;
        betType = 'R';
    } else if winProbability > 1.1 / (activePlayerCount as f64) {
        bet = _table.minimum_bet;
        betType = 'C';
    } else if winProbability > 0.9 / (activePlayerCount as f64) && us.bet == 0 && _table.minimum_bet == 20 {
        bet = _table.minimum_bet;
        betType = 'C';
    } else {
        bet = 0;
        betType = 'F';
    }

    println!("Opponent count: {}, community cards: {}, win probability: {:.1}%, bet: {}{} simulation time: {:?}", activePlayerCount - 1, communityCards.len(), winProbability * 100.0, betType, bet, simulationTime);

    return crate::models::bet::Bet{bet};

    //println!("Table: {:?}", _table);

    // number of hidden community cards
    let mut hidden_community_cards: i32 = 5 - _table.community_cards.len() as i32;

    // vector of card values
    let mut number: Vec<i32> = vec![0; 13];

    // boolean vector of card values
    let mut straight: Vec<bool> = vec![false; 13];
    
    // vector of card colors
    let mut color: Vec<(crate::models::suit::Suit, i32)> = vec![(crate::models::suit::Suit::HEARTS, 0), (crate::models::suit::Suit::SPADES, 0), (crate::models::suit::Suit::CLUBS, 0), (crate::models::suit::Suit::DIAMONDS, 0)];

    // fill number vec
    for _player in &_table.players {
        if _player.name == "Texas Hold'team" {
            for _card_vec in &_player.cards {
                for _card in _card_vec {
                    match _card.rank {
                        crate::models::rank::Rank::A => number[12] += 1,
                        crate::models::rank::Rank::K => number[11] += 1,
                        crate::models::rank::Rank::Q => number[10] += 1,
                        crate::models::rank::Rank::J => number[9] += 1,
                        crate::models::rank::Rank::_10 => number[8] += 1,
                        crate::models::rank::Rank::_9 => number[7] += 1,
                        crate::models::rank::Rank::_8 => number[6] += 1,
                        crate::models::rank::Rank::_7 => number[5] += 1,
                        crate::models::rank::Rank::_6 => number[4] += 1,
                        crate::models::rank::Rank::_5 => number[3] += 1,
                        crate::models::rank::Rank::_4 => number[2] += 1,
                        crate::models::rank::Rank::_3 => number[1] += 1,
                        crate::models::rank::Rank::_2 => number[0] += 1,
                    }
                }
            }

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
                crate::models::rank::Rank::A => number[12] += 1,
                crate::models::rank::Rank::K => number[11] += 1,
                crate::models::rank::Rank::Q => number[10] += 1,
                crate::models::rank::Rank::J => number[9] += 1,
                crate::models::rank::Rank::_10 => number[8] += 1,
                crate::models::rank::Rank::_9 => number[7] += 1,
                crate::models::rank::Rank::_8 => number[6] += 1,
                crate::models::rank::Rank::_7 => number[5] += 1,
                crate::models::rank::Rank::_6 => number[4] += 1,
                crate::models::rank::Rank::_5 => number[3] += 1,
                crate::models::rank::Rank::_4 => number[2] += 1,
                crate::models::rank::Rank::_3 => number[1] += 1,
                crate::models::rank::Rank::_2 => number[0] += 1,
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

        let fucking_color_shit = color.clone();


    // find card with highest value in number, reassign for same-size larger-index
    let mut highest_card_value: i32 = 0;
    let mut highest_card_count: i32 = 0;

    for i in 0..13 {
        if number[i] >= highest_card_count {
            highest_card_count = number[i];
            highest_card_value = i as i32;

        }
    }

    // find card with second-highest value in number, reassign for same-size larger-index
    let mut second_highest_card_value: i32 = 0;
    let mut second_highest_card_count: i32 = 0;

    for i in 0..13 {
        if (number[i] >= second_highest_card_value) && (i as i32 != highest_card_value) {
            second_highest_card_count = number[i];
            second_highest_card_value = i as i32;
        }
    }

    // check all windows of 5 cards in vector straight and find the window with the fewest false values
    let mut min_missing_cards: i32 = 6;

    for i in 0..9 {
        let mut min_missing_cards_temp: i32 = 0;
        for j in i..i+5 {
            if !straight[j] {
                min_missing_cards_temp += 1;
            }
        }

        if min_missing_cards_temp < min_missing_cards {
            min_missing_cards = min_missing_cards_temp;
        }
    }

// find maximum number of cards in color
    let mut max_color: i32 = 0;
    for c in color {
        if c.1 > max_color {
            max_color = c.1;
        }
    }
    
    // all possible combinations in descending order of worth

    let binding = _table.players.clone().into_iter().filter(|player| player.name == "Texas Hold'team").collect::<Vec<Player>>();
    let us: Option<&Player> = binding.first();
    let us = us.unwrap();
    let min_bet = _table.minimum_bet;
    let min_raise = _table.minimum_raise;
    
    // fuck royal flush and straight flush, nobody gets that and we go all-in anyways
    let four_of : bool = highest_card_count >= 4;
    let full_house: bool = highest_card_count >= 3 && second_highest_card_count >= 2;
    let flush: bool = max_color >= 5;

    let mut flush_missing : i32 = 8;
    if !flush {
       flush_missing = 5 - max_color;
    }

    let straight: bool = min_missing_cards == 0;
    let three_of    : bool = highest_card_count >= 3;
    let two_pairs : bool = highest_card_count >= 2 && second_highest_card_count >= 2;
    // fuck pair highest_card, worthless shit

    let active_player_count = _table.players.clone().into_iter().filter(|player| player.status != crate::models::player::PlayerStatusEnum::OUT).collect::<Vec<Player>>().len();

    let players = _table.players.clone().into_iter().filter(|player| player.name != "Texas Hold'team").collect::<Vec<Player>>();
    let mut max_opponent_stack: i32 = 0;
    for p in players {
        if (p.stack > max_opponent_stack) {
            max_opponent_stack = p.stack;
        }
    } 

    let nemesis_binding = _table.players.clone().into_iter().filter(|player| player.name == "Team42").collect::<Vec<Player>>();
    let nemesis: Option<&Player> = nemesis_binding.first();
    
    let nemesis_bet = nemesis.unwrap().bet;

    let nemesis_raise : bool = nemesis_bet > min_bet;

    let nemesis_all_in : bool = nemesis_bet == nemesis.unwrap().stack;

    let max_bet = max(min(max_opponent_stack, us.stack), min_bet);


    let mut highest_card: i32 = 0;
    for i in 0..13 {
        if number[i] != 0 {
            highest_card = i as i32;
        }

        }

    let mut encounters: i32 = 0;
    let mut shit_starting_hand: bool = true;

    let mut chen_score: f32 = 0.0;
    
     
    if (hidden_community_cards == 5) {
        // 1. score highest card
        if (highest_card_value == 12) {
            chen_score += 10.0
        }
        else if (highest_card_value == 11) {
            chen_score += 8.0
        }
        else if (highest_card_value == 10) {
            chen_score += 7.0
        }
        else if (highest_card_value == 9) {
            chen_score += 6.0
        }
        else {
            chen_score += highest_card_value as f32 / 2.0
        }

        // 2. pair points
        if (highest_card_count == 2) {
            chen_score = chen_score * 2.0;

            if (chen_score < 5.0) {
                chen_score = 5.0;
            }
        }

        // 3. add points for suited cards
        if max_color == 2 { 
            chen_score += 2.0;
        }

        // 4. subtract points for gaps
        let mut abstand = 69;

        if (highest_card_count != 2) {

            let mut i_abs = -100;
                    for i in (0..13) {
                        if number[i] ==1  && i_abs == -100{
                            i_abs = -(i as i32);
                        }
                        else if number[i] > 0{
                            i_abs = i_abs + (i as i32);
                        }
                    }
                    
                    i_abs -= 1;

                    if i_abs == 1 {
                        chen_score -= 1.0;
                    }
                    else if i_abs == 2 {
                        chen_score -= 2.0;
                    }
                    else if i_abs == 3 {
                        chen_score -= 4.0;
                    }
                    else if i_abs >=4 {
                        chen_score -= 5.0;
                    }

                    abstand = i_abs;
        }

        // 5. gap 0/1 and both cards < 10
       if (abstand < 2) && highest_card_value < 10 {
            chen_score += 1.0;
       }

       chen_score = (chen_score - 0.1).ceil();
        
        if (chen_score >= 6.0) {
            shit_starting_hand = false;
        }

        println!("CHEN:{}", chen_score);

    }

    
    let mut we_have_not_complete_shit: bool = false;

    if (highest_card_count >= 2 && highest_card_value >= 10 || highest_card >= 12) {
        we_have_not_complete_shit = true;
    }

    print!("Active players: {}, community cards: {}", active_player_count, _table.community_cards.len() as i32);

    let mut bet = 0;

    if (active_player_count < 8 || chen_score >= 12.0) {

        // good cards -> go completely bonkers
        if (four_of || full_house || flush || straight) {
            println!(" -> go bonkers (four_of: {}, full_house: {}, flush: {}, straight: {})", four_of, full_house, flush, straight);
            bet = max_bet;
        }
        // not so bad, stay in
        else if (three_of || two_pairs) {
            if nemesis_all_in {
                println!(" -> not so bad, but fold because nemesis all in");
                bet = 0;
            } else {
                println!(" -> not so bad");
                bet = min_raise;
            }
        }
        // if there are unopened center cards left: hope for something
        else if ((min_missing_cards <= 2 || flush_missing <= 2 ) && hidden_community_cards >= 3) {
            if nemesis_raise {
                println!(" -> hope, but fold because nemesis raise");
                bet = 0;
            } 
            else if (active_player_count <= 3 && we_have_not_complete_shit) {
                println!(" -> hope, raise because player count and not complete shit");
                bet = min_raise;
            }
            else {
                println!(" -> hope, continue");
                bet = min_bet;
            }
        }
        else if (active_player_count <= 3 && we_have_not_complete_shit) {
            println!(" -> min bet because player count and not complete shit");
            bet = min_bet;
        }
        else if (hidden_community_cards == 5 && !shit_starting_hand) {
            println!(" -> min bet because round start and not shit starting hand");
            bet = min_bet;
        }
        else if (hidden_community_cards == 3 || hidden_community_cards == 4) {
            println!(" -> min bet because mid game");
            bet = min_bet;
        }
        // sometimes give up
        else {
            println!(" -> never give up");
            {
                bet = min_bet;
            }
        }
    }

    println!("Wir sind die Besten!");


    return crate::models::bet::Bet{bet: bet};

}


fn simulateWinProbability(hand: &Vec<Card>, communityCards: &Vec<Card>, activeOpponentCount: i32) -> f64 {
    // create deck with all cards which are not in the given cards
    let mut deck: Vec<Card> = vec![];
    for rank in Rank::iter() {
        for suit in Suit::iter() {
            let card: Card = Card {rank, suit};
            if !hand.contains(&card) && !communityCards.contains(&card) {
                deck.push(card);
            }
        }
    }

    let mut rng = thread_rng();

    // simulate n games
    let n: i32 = 100000;
    let mut wins: i32 = 0;
    'sim: for _i in 0..n {
        let mut deckTmp: Vec<Card> = deck.clone();

        // shuffle remaining cards in deck to enable random card drawing
        deckTmp.shuffle(&mut rng);

        // draw remaining community cards from the deck
        let mut allCommunityCards: Vec<Card> = communityCards.clone();
        for _j in 0..(5 - communityCards.len()) {
            allCommunityCards.push(deckTmp.pop().unwrap());
        }

        let mut ownCards: Vec<Card> = allCommunityCards.clone();
        ownCards.extend(hand);
        let ownScore: i32 = evaluateHand(&ownCards);

        for _j in 0..activeOpponentCount {
            let mut opponentCards: Vec<Card> = allCommunityCards.clone();
            opponentCards.extend(vec![deckTmp.pop().unwrap(), deckTmp.pop().unwrap()]);
            
            let opponentScore: i32 = evaluateHand(&opponentCards);
            if opponentScore >= ownScore {
                continue 'sim;
            }
        }

        wins += 1;
    }

    return (wins as f64) / (n as f64);
}


fn evaluateHand(cards: &Vec<Card>) -> i32 {
    // count number of cards of each rank
    let mut rankCount: Vec<i32> = vec![0; 13];
    for card in cards {
        rankCount[card.rank as usize] += 1;
    }

    // check for straight/royal flush
    let mut straightFlushCount: i32 = 0;
    for i in (0..13).rev() {
        if rankCount[i] > 0 {
            straightFlushCount += 1;
        } else {
            straightFlushCount = 0;
        }

        if straightFlushCount == 5 {
            return 800 + (i as i32);
        }
    }

    // find highest count of cards of the same rank
    let mut mostCommonRank: i32 = 12;
    for i in (0..12).rev() {
        if rankCount[i] > rankCount[mostCommonRank as usize] {
            mostCommonRank = i as i32;
        }
    }

    // check for four of a kind
    if rankCount[mostCommonRank as usize] == 4 {
        return 700 + mostCommonRank;
    }

    // find second highest count of cards of the same rank
    let mut secondMostCommonRank: i32 = if mostCommonRank != 12 { 12 } else { 11 };
    for i in (0..12).rev() {
        if rankCount[i] > rankCount[secondMostCommonRank as usize] && i as i32 != mostCommonRank {
            secondMostCommonRank = i as i32;
        }
    }

    // check for full house
    if rankCount[mostCommonRank as usize] == 3 && rankCount[secondMostCommonRank as usize] == 2 {
        return 600 + mostCommonRank;
    }

    // count number of cards of each suit
    let mut suitCount: Vec<i32> = vec![0; 4];
    for card in cards {
        suitCount[card.suit as usize] += 1;
    }

    // find highest count of cards of the same suit
    let mut mostCommonSuit: i32 = 0;
    for i in 1..4 {
        if suitCount[i] > suitCount[mostCommonSuit as usize] {
            mostCommonSuit = i as i32;
        }
    }

    // check for flush
    if suitCount[mostCommonSuit as usize] >= 5 {
        for card in cards.iter().rev() {
            if card.suit as i32 == mostCommonSuit {
                return 500 + (card.rank as i32);
            }
        }
    }

    // check for straight
    let mut straightCount: i32 = 0;
    for i in (0..13).rev() {
        if rankCount[i] > 0 {
            straightCount += 1;
        } else {
            straightCount = 0;
        }

        if straightCount == 5 {
            return 400 + (i as i32);
        }
    }

    // check for three of a kind
    if rankCount[mostCommonRank as usize] == 3 {
        return 300 + mostCommonRank;
    }

    // check for two pairs
    if rankCount[mostCommonRank as usize] == 2 && rankCount[secondMostCommonRank as usize] == 2 {
        return 200 + mostCommonRank;
    }

    // check for pair
    if rankCount[mostCommonRank as usize] == 2 {
        return 100 + mostCommonRank;
    }

    // return highest card
    return mostCommonRank;
}
