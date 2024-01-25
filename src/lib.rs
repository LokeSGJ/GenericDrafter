#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct Player {
    number: usize,
    picks: Vec<String>,
}

//TODO: read_file panics if the file path is bad
//TODO: generate_picks panics if unique_picks is true and file contains fewer options than num_players * num_picks
fn run_drafter(path: &mut String, num_players: usize, num_picks: usize, unique_picks: bool) -> Vec<Player> {
    let mut options: Vec<String> = Vec::new();
    read_file(path, &mut options).expect("Failed to read file");

    let pick_list : Vec<String> = generate_picks(&num_players, &num_picks, options, unique_picks);

    let player_list : Vec<Player> = generate_players(num_players, &num_picks, pick_list);

    //print_draft(player_list, num_picks);

    return player_list;
}

fn generate_picks (num_players: &usize, num_picks: &usize, mut options: Vec<String>, unique_picks: bool) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut pick_list: Vec<String> = Vec::new();

    if unique_picks {
        for _i in 0..num_players * num_picks {
            let index: usize = rng.gen_range(0..options.len());
            let pick: String = options[index].to_string();

            options.remove(index);

            pick_list.push(pick);
        }
    }
    else if !unique_picks { //I'd like to officially apologize for this part
        for _i in 0..*num_players {
            let mut picks: Vec<String> = Vec::new();

            for _i in 0..*num_picks {
                let index: usize = rng.gen_range(0..options.len());
                let mut pick: String = options[index].to_string();

                while picks.contains(&pick) {
                    let index: usize = rng.gen_range(0..options.len());
                    pick = options[index].to_string();
                }

                picks.push(pick);
            }

            for i in 0..*num_picks {
                pick_list.push(picks[i].as_str().to_string());
            }
        }
    }

    return pick_list;
}

fn generate_players (num_players: usize, num_picks: &usize, pick_list: Vec<String>) -> Vec<Player> {
    let mut player_list: Vec<Player> = Vec::new();
    for i in 0..num_players {
        let mut picks: Vec<String> = Vec::new();
        for j in 0..*num_picks{
            let pick: &String = &pick_list[i*num_picks+j];
            picks.push(pick.to_string());
        }
        let player = Player {
            number: i+1,
            picks,
        };
        player_list.push(player);
    }

    return player_list;
}

//print_draft was useful for early testing but doesn't actually serve a purpose anymore
fn print_draft (player_list: Vec<Player>, num_picks: usize) {
    for i in player_list {
        println!("Player {} chooses between:\n", i.number);
        for j in 0..num_picks {
            println!("{}\n", i.picks[j]);
        }
    }
}

fn read_file (path: &mut String, options: &mut Vec<String>) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        options.push(line);
    }

    Ok(())
}