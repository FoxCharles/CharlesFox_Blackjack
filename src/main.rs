use rand::prelude::*;
use std::io;

#[derive(Clone)]
struct Card {
    value: String,
    suit: String,
}

impl Card {
    fn new(value: String, suit: String) -> Card {
        Card{value, suit}
    }
    fn get_value(&self) -> u64 {
        match self.value.as_ref() {
            "A" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "J" => 10,
            "Q" => 10,
            "K" => 10,
            _ => 0
        }
    }
}

struct Deck {
    stack: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let stack = Vec::new();
        Deck{stack}
    }
    
    fn populate(&mut self) {
        let mut stack = Vec::new();
        for i in 0..4 {
            for j in 0..13 {
                let gen_value = match j {
                    0 => "A",
                    1 => "2",
                    2 => "3",
                    3 => "4",
                    4 => "5",
                    5 => "6",
                    6 => "7",
                    7 => "8",
                    8 => "9",
                    9 => "10",
                    10 => "J",
                    11 => "Q",
                    12 => "K",
                    _ => "",
                    
                };
                let gen_suit = match i {
                    0 => "♠",
                    1 => "♥",
                    2 => "♦",
                    3 => "♣",
                    _ => "",
                };
                stack.push(Card::new(gen_value.to_owned(), gen_suit.to_owned()));
            }
        }
        self.stack = stack;
    }
    
    fn shuffle(&mut self) {
        for _i in 0..100 {
            let mut shuffle_stack_1 = Vec::new();
            let mut shuffle_stack_2 = Vec::new();
            for card in &self.stack {
                if random() {
                    shuffle_stack_1.push(card.clone());
                }
                else {
                    shuffle_stack_2.push(card.clone());
                }
            }    
            shuffle_stack_1.append(&mut shuffle_stack_2);
            self.stack = shuffle_stack_1;
        }
    }
    
    fn print_deck(&self) -> String {
        let mut message = String::new();
        for card in &self.stack {
            message.push_str(&card.value);            
            message.push_str(&card.suit);
            message.push_str(" ");
        }
        message
    }
    
    fn count_deck(&self, ace_value:u64) -> u64 {
        let mut total = 0u64;
        for card in &self.stack {
            let card_value = card.get_value();
            if card_value == 1 {
                total += ace_value;
            }
            else {
                total += card_value;
            }
        }
        total
    }
}





#[test]
fn test_populate() {
    let mut deck = Deck::new();
    deck.populate();
    assert_eq!(deck.stack.len(), 52);
}

#[test]
fn test_count() {
    let mut deck = Deck::new();
    deck.stack.push(Card{value:"A".to_string(), suit:"♠".to_string()});
    deck.stack.push(Card{value:"9".to_string(), suit:"♦".to_string()});
    deck.stack.push(Card{value:"K".to_string(), suit:"♣".to_string()});
    assert_eq!(deck.count_deck(1), 20);
    assert_eq!(deck.count_deck(11), 30);
}

//Can't really have a test case for shuffle, since the point of it is that it's random, but we can make sure it doesn't mess with the deck's length at least.
#[test]
fn test_shuffle() {
    let mut deck = Deck::new();
    deck.populate();
    deck.shuffle();
    assert_eq!(deck.stack.len(), 52);
}

#[test]
fn test_print_deck() {
    let mut deck = Deck::new();
    deck.stack.push(Card{value:"A".to_string(), suit:"♠".to_string()});
    deck.stack.push(Card{value:"9".to_string(), suit:"♦".to_string()});
    deck.stack.push(Card{value:"K".to_string(), suit:"♣".to_string()});
    let message = deck.print_deck();
    assert_eq!(&message, "A♠ 9♦ K♣ ");
}




fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            input = input.trim().to_string();
        },
        Err(error) => println!("error: {}", error),
    }     
    input
}

fn main() {
    clear_screen();
    let mut difficulty = 0;
    let mut cheat_percent = 0;
    let mut card_limit = 18;
    println!("Welcome to this game of Blackjack! There are four difficulty options.\n1. Play a game with your cousin Lenny. He's new to Blackjack, and thus kind of terrible at it.\n2. Play a game with some pals at a local card club. They're familiar with the game but not experts at it.\n3. Play a game of Blackjack at your local casino. You're up against an expert!\n4. Play a game against a local clairvoyant. She knows what the cards are ahead of time, or at least claims to.\nPick 1-4.");
    let mut invalid = true;
    while invalid {
        invalid = false;
        match get_input().as_str() {                
            "1" => {
                difficulty = 1;
                cheat_percent = 0;
                card_limit = 19;
            },
            "2" => {
                difficulty = 2;
                cheat_percent = 15;
                card_limit = 18;
            },
            "3" => {
                difficulty = 3;
                cheat_percent = 50;
                card_limit = 18;
            },
            "4" => {
                difficulty = 4;
                cheat_percent = 100;
                card_limit = 18;
            },
            _ => {
                println!("Please input a valid number.");
                invalid = true;
            }
        }
    }
    let mut playing = true;
    let mut main_deck = Deck::new();
    let mut player_deck = Deck::new();
    let mut dealer_deck = Deck::new();    
    let mut discard_deck = Deck::new();
    main_deck.populate();
    main_deck.shuffle();
    handle_dialog("intro".to_string(), difficulty);
    println!("(press <enter> to continue)");
    get_input();
    while playing {
        gamelogic (difficulty, cheat_percent, card_limit, &mut main_deck, &mut player_deck, &mut dealer_deck, &mut discard_deck);
        println!("Play again? y/n");
        let mut invalid = true;
        while invalid {
            match get_input().as_str() {
                "y" => {
                    playing = true;
                    invalid = false;
                },                    
                "n" => {
                    playing = false;                               
                    invalid = false;
                },
                _ => println!("Please input a valid number."),
            }
        }
    }
    handle_dialog("quit".to_string(), difficulty);
}

fn gamelogic(difficulty:u64, cheat_percent:u64, card_limit:u64, main_deck:&mut Deck, player_deck:&mut Deck, dealer_deck:&mut Deck, discard_deck:&mut Deck) {
    //Start round, hand out two cards each
    for i in 0..4 {
        if main_deck.stack.len() == 0 {
            for _i in 0..discard_deck.stack.len() {
                main_deck.stack.push(discard_deck.stack.pop().unwrap());
                main_deck.shuffle();
            }
        }
        if i % 2 == 0 {
            player_deck.stack.push(main_deck.stack.pop().unwrap());
        }
        else {
            dealer_deck.stack.push(main_deck.stack.pop().unwrap());
        }
    }    
    //loop
    let mut game_running = true;
    let mut player_playing = true;
    let mut dealer_playing = true;
    let mut winner = 0;
    let mut msgtype = String::new();
    while game_running {
        msgtype = String::new();
        clear_screen();
        println!("Your cards: {}",player_deck.print_deck());
        print!("Dealer's Cards: {}{} ",dealer_deck.stack[0].value,dealer_deck.stack[0].suit);
        for _i in 1..dealer_deck.stack.len() {
            print!("** ");
        }
        println!("");
        //check for winners
        if player_deck.count_deck(1) == 21 || player_deck.count_deck(11) == 21 {
            clear_screen();
            println!("Your cards: {}",player_deck.print_deck());
            println!("Dealer's cards: {}",dealer_deck.print_deck());
            msgtype.push_str("youblackjack");
            winner = 1;
            player_playing = false;            
            dealer_playing = false;
            game_running = false;
        }
        else if dealer_deck.count_deck(1) == 21 || dealer_deck.count_deck(11) == 21 {
            clear_screen();
            println!("Your cards: {}",player_deck.print_deck());
            println!("Dealer's cards: {}",dealer_deck.print_deck());
            msgtype.push_str("theyblackjack");
            winner = 2;
            player_playing = false;            
            dealer_playing = false;
            game_running = false;
        }
        //player chooses
        if player_playing {
            println!("<H>it or <S>tay?");
            let mut invalid = true;
            while invalid {
                invalid = false;
                match get_input().as_str() {
                    "h"|"H" => {
                        if main_deck.stack.len() == 0 {
                            for _i in 0..discard_deck.stack.len() {
                                main_deck.stack.push(discard_deck.stack.pop().unwrap());
                                main_deck.shuffle();
                            }
                        }
                        player_deck.stack.push(main_deck.stack.pop().unwrap());                        
                    },                    
                    "s"|"S" => {
                        player_playing = false;
                    },
                    _ => {
                        println!("Please input a valid response.");                        
                        invalid = true;
                    },
                }
            }  
            if player_deck.count_deck(1) > 21 {
                clear_screen();
                println!("Your cards: {}",player_deck.print_deck());
                println!("Dealer's cards: {}",dealer_deck.print_deck());
                msgtype.push_str("youbust");
                winner = 2;
                player_playing = false;            
                dealer_playing = false;
                game_running = false;
            }
        }
        //check for winners
        if player_deck.count_deck(1) == 21 || player_deck.count_deck(11) == 21 {
            clear_screen();
            println!("Your cards: {}",player_deck.print_deck());
            println!("Dealer's cards: {}",dealer_deck.print_deck());
            msgtype = String::new();
            msgtype.push_str("youblackjack");
            winner = 1;
            player_playing = false;            
            dealer_playing = false;
            game_running = false;
        }
        else if dealer_deck.count_deck(1) == 21 || dealer_deck.count_deck(11) == 21 {
            clear_screen();
            println!("Your cards: {}",player_deck.print_deck());
            println!("Dealer's cards: {}",dealer_deck.print_deck());
            msgtype = String::new();
            msgtype.push_str("theyblackjack");
            winner = 2;
            player_playing = false;            
            dealer_playing = false;
            game_running = false;
        }
        //dealer chooses
        if dealer_playing {
            if main_deck.stack.len() == 0 {
                for _i in 0..discard_deck.stack.len() {
                    main_deck.stack.push(discard_deck.stack.pop().unwrap());
                    main_deck.shuffle();
                }
            }
            let mut rng = thread_rng();
            let n = rng.gen_range(1, 100);
            if n <= cheat_percent {
                //time to cheat!
                let dealer_value = dealer_deck.count_deck(1);
                let next_card = main_deck.stack.pop().unwrap();
                let potential_value = dealer_value+next_card.get_value();
                if potential_value <= 21 || potential_value+10 == 31 {
                    dealer_deck.stack.push(next_card);
                }
                else {
                    main_deck.stack.push(next_card);
                    dealer_playing = false;
                }
            }
            else {
                //Play it cool
                let dealer_value = dealer_deck.count_deck(1);
                if dealer_value <= card_limit {
                    if main_deck.stack.len() == 0 {
                        for _i in 0..discard_deck.stack.len() {
                            main_deck.stack.push(discard_deck.stack.pop().unwrap());
                            main_deck.shuffle();
                        }
                    }
                    dealer_deck.stack.push(main_deck.stack.pop().unwrap());  
                }
                else {
                    dealer_playing = false;
                }
            }   
            if dealer_deck.count_deck(1) > 21 {
                clear_screen();
                println!("Your cards: {}",player_deck.print_deck());
                println!("Dealer's cards: {}",dealer_deck.print_deck());
                msgtype.push_str("theybust");
                winner = 1;
                player_playing = false;            
                dealer_playing = false;
                game_running = false;
            }            
        }
        if !dealer_playing && !player_playing {
            game_running = false;
        }
    }
    //end of match
    if winner == 0 {
        clear_screen();
        println!("Your cards: {}",player_deck.print_deck());
        println!("Dealer's cards: {}",dealer_deck.print_deck());
        //compare scores. Dealer always wins.
        let mut player_score = player_deck.count_deck(11);
        if player_score > 21 {
            player_score -= 10;
        }
        let mut dealer_score = dealer_deck.count_deck(11);
        if dealer_score > 21 {
            dealer_score -= 10;
        }
        match dealer_score >= player_score {
            true => winner = 2,
            false => winner = 1,
        }       
        //1 = player, 2 = dealer 
        match winner {
            1 => {
                msgtype.push_str("youwin");
            },
            2 => {
                msgtype.push_str("theywin");
            },
            _ => {},
        }
    }
    for _i in 0..player_deck.stack.len() {
        discard_deck.stack.push(player_deck.stack.pop().unwrap());   
    }
    for _i in 0..dealer_deck.stack.len() {
        discard_deck.stack.push(dealer_deck.stack.pop().unwrap());
    }
    handle_dialog(msgtype, difficulty);
}

fn handle_dialog(msgtype:String, difficulty:u64) {
    match msgtype.as_str() {
        "intro" => {
            match difficulty {
                1 => println!("\"Oh, man, so I just hand out one--no, two cards to each of us, right? This is so cool, thanks for letting me play with you! I hope I win!\""),
                2 => println!("You'd sit down across from a redhead named Wilma, and she'd smile, starting to pass out the four cards. \"Good luck.\""),
                3 => println!("The dealer quietly and professionally hands out four cards each."),
                4 => println!("\"Welcome to Madam Lulu's fortune telling. This... card game idea intrigues me, if you are absolutely positive you wish someone who can see the future as an opponent, then let the whims of fate decide the winner.\""),
                _ => {}
            }
        },
        "youbust" => {
            match difficulty {
                1 => println!("You turn over your bust deck, and Lenny laughs. \"Haha! I won! Let's do it again, let's do it again!\""),
                2 => println!("You turn over your bust deck, and Wilma shakes her head. \"Bad luck. Happens to the best of us.\""),
                3 => println!("You turn over your bust deck, and the dealer shrugs. \"Happens.\""),
                4 => println!("You turn over your bust deck, and Madam Lulu nods her head. \"As I foresaw.\""),
                _ => {}
            }
        },
        "theybust" => {
            match difficulty {
                1 => println!("Lenny looks disappointed as he shows you his bust deck. \"That's... over 21. Does that mean I win? I lose? Aww.\""),
                2 => println!("Wilma turns over her bust deck. \"Darn. One more game?\""),
                3 => println!("The dealer turns over his bust deck. \"I bust.\""),
                4 => println!("Madam Lulu tilts her head, looking over her bust deck. \"This literally shouldn't be possible. This is a bug in this program, you'd better tell someone about it.\""),
                _ => {}
            }
        },
        "youblackjack" => {
            match difficulty {
                1 => println!("You flourish your blackjack, and Lenny looks confused. \"That means... you won? Darnit, one more time, I swear I'll win!\""),
                2 => println!("You flourish your blackjack, and Wilma smiles. \"Good bit of luck there. One more?\""),
                3 => println!("You flourish your blackjack, and the dealer just gives you a look and a sarcastic, \"Congratulations.\""),
                4 => println!("You flourish your blackjack, and Madam Lulu nods her head approvingly, \"Seems the spirits were on your side this time.\""),
                _ => {}
            }
        },
        "theyblackjack" => {
            match difficulty {
                1 => println!("Lenny shows you his blackjack. \"This is 21. That's the... goal, right? I won? Woo!\""),
                2 => println!("Wilma turns over her blackjack. \"Blackjack. One more game?\""),
                3 => println!("The dealer gives you a smug look as he reveals his blackjack. \"Bad luck.\""),
                4 => println!("Madam Lulu shows you her blackjack, \"Fate meant this to happen.\""),
                _ => {}
            }
        },
        "youwin" => {
            match difficulty {
                1 => println!("Lenny doublechecks the count of the cards on his fingers. \"You got higher than me? Aww. One more time, I think I'm getting the hang of this!\""),
                2 => println!("Wilma nods her head. \"Congratulations, you knew when to hold.\""),
                3 => println!("The dealer counts the two sets of cards, nodding his head. \"You win.\""),
                4 => println!("Madam Lulu smiles, looking over the two sets of cards. \"I predicted your win this time, you know. I just didn't say it.\""),
                _ => {}
            }
        },
        "theywin" => {
            match difficulty {
                1 => println!("Lenny is practically beaming. \"Mine's bigger than yours, mine's bigger than yours! One more time, I'm on a roll!\""),
                2 => println!("Wilma smiles, looking over her larger cards. \"Can't win them all. Want to play again?\""),
                3 => println!("The dealer counts the two sets of cards, smirking. \"I win.\""),
                4 => println!("Madam Lulu counts the two sets of cards, nodding her head. \"My victory was fate.\""),
                _ => {}
            }
        },
        "quit" => {
            match difficulty {
                1 => println!("Lenny looks disappointed. \"Done for now? Okay, we can continue this tomorrow.\""),
                2 => println!("Wilma nods. \"Thanks for dropping by the club, I'll see you later.\""),
                3 => println!("The dealer merely gives you a nod and moves on to the next player."),
                4 => println!("\"This was... enlightening. Drop by anytime you wish to test fate.\""),
                _ => {}
            }
        },
        _ => {},
    }
}