use rocket::serde::json::Json;

pub fn decide(_table: Json<crate::models::table::Table>) -> crate::models::bet::Bet {
    // Look over all players and find the one with the name textas hold'team
    let mut cardsvec = Vec::new();
    for player in &_table.players {
        if player.name == "Texas Hold'team" {
            // If the player has cards, then we can check them
            let cards = player.cards.clone();
            if let Some(cards) = cards {
                cardsvec = cards;
            }
        }
    }

    let mut communityCards = _table.community_cards.clone();
    cardsvec.append(&mut communityCards);
    let cardslen = cardsvec.len();

    let mut sum = 0;
    let mut boolsuit = true;
    for i in 0..cardslen - 1 {
        // Add up all card ranks
        //let card_num = match cardsvec[i].rank {
        //    crate::models::rank::Rank::A => 14,
        //    crate::models::rank::Rank::K => 13,
        //    crate::models::rank::Rank::Q => 12,
        //    crate::models::rank::Rank::J => 11,
        //    crate::models::rank::Rank::_10 => 10,
        //    crate::models::rank::Rank::_9 => 9,
        //    crate::models::rank::Rank::_8 => 8,
        //    crate::models::rank::Rank::_7 => 7,
        //    crate::models::rank::Rank::_6 => 6,
        //    crate::models::rank::Rank::_5 => 5,
        //    crate::models::rank::Rank::_4 => 4,
        //    crate::models::rank::Rank::_3 => 3,
        //    crate::models::rank::Rank::_2 => 2,
        //};
        //sum += card_num;

        if cardsvec[i].rank == cardsvec[i + 1].rank {
            // If the two cards are the same, then we can raise
            sum += 1;
        }
        if cardsvec[i].suit != cardsvec[cardslen - 1].suit {
            // If the two cards are the same suit, then we can raise
            //sum += 1;
            boolsuit = false;
        }
    }

    if boolsuit {
        sum += 1;
    }

    //let normSum = sum / cardslen;
    if sum >= 3 && cardslen >= 5 {
        return crate::models::bet::Bet {
            bet: _table.minimum_raise,
        };
    } else {
        return crate::models::bet::Bet {
            bet: _table.minimum_bet,
        };
    }
}
