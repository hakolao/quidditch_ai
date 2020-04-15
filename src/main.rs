use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

static BOUNDS: &'static [(i32, i32)] = &[(0, 0), (16001, 0), (0, 7501), (7501, 16001)];

struct Snaffle {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

impl Snaffle {
    pub fn new(id: i32, x: i32, y: i32, vx: i32, vy: i32) -> Snaffle {
        Snaffle { id, x, y, vx, vy }
    }

    pub fn get_destination(&self) -> (i32, i32) {}
}

struct Wizard {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
    pub has_sniffle: bool,
    pub is_opponent: bool,
}

impl Wizard {
    pub fn new(id: i32, x: i32, y: i32, vx: i32, vy: i32, has_sniffle: bool, is_opponent: bool) -> Wizard {
        Wizard { id, x, y, vx, vy, has_sniffle, is_opponent }
    }

    pub fn get_destination(&self) -> (i32, i32) {}
}

// if 0 you need to score on the right of the map, if 1 you need to score on the left 
fn parse_team_id() -> i32 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_team_id = parse_input!(input_line, i32);
    my_team_id
}

fn parse_loop_variables() -> (i32, i32, i32, i32, i32) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let my_score = parse_input!(inputs[0], i32);
    let my_magic = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let opponent_score = parse_input!(inputs[0], i32);
    let opponent_magic = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let entities = parse_input!(input_line, i32); // number of entities still in game
    (my_score, my_magic, opponent_score, opponent_magic, entities)
}


fn parse_entity_variables() -> (i32, String, i32, i32, i32, i32, bool) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let entity_id = parse_input!(inputs[0], i32); // entity identifier
    let entity_type = inputs[1].trim().to_string(); // "WIZARD", "OPPONENT_WIZARD" or "SNAFFLE" or "BLUDGER"
    let x = parse_input!(inputs[2], i32); // position
    let y = parse_input!(inputs[3], i32); // position
    let vx = parse_input!(inputs[4], i32); // velocity
    let vy = parse_input!(inputs[5], i32); // velocity
    // 1 if the wizard is holding a Snaffle, 0 otherwise. 1 if the Snaffle is being held, 0 otherwise. id of the last victim of the bludger.
    let hasSniffle = parse_input!(inputs[6], i32);
    (entity_id, entity_type, x, y, vx, vy, has_snaffle as bool)
}

fn find_most_desirable_snaffle(wizard: &Wizard, snaffles: &Vec<Snaffle>) -> Snaffle {}

/**
 * Grab Snaffles and try to throw them through the opponent's goal!
 * Move towards a Snaffle to grab it and use your team id to determine towards where you need to throw it.
 * Use the Wingardium spell to move things around at your leisure, the more magic you put it, the further they'll move.
 **/
fn main() {
    let my_team_id = parse_team_id();

    // game loop
    loop {
        let (my_score, my_magic, opponent_score, opponent_magic, entities) = parse_loop_variables();
        let mut my_wizards = vec![];
        let mut opponent_wizards = vec![];
        let mut snaffles = vec![];
        for i in 0..entities as usize {
            let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
            if &entity_type == "WIZARD" {
                my_wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, false));
            }
            if &entity_type == "OPPONENT_WIZARD" {
                opponent_wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, true));
            }
            if &entity_type == "SNAFFLE" {
                snaffles.push(Snaffle::new(entity_id, x, y, vx, vy));
            }
        }
        for wizard in my_wizards.iter() {
            let action = if has_sniffle {
                let x_to = ;
                let y_to = ;
                let power = ;
                format!("{} {} {} {}", "THROW", x_to, y_to, power);
            } else {
                let snaffle = find_most_desirable_snaffle(wizard, &snaffles);
                let (x_to, y_to) = snaffle.get_destination();
                let thrust = wizard.;
                format!("{} {} {} {}", "MOVE", x_to, y_to, thrust);
            }
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");


            // Edit this line to indicate the action for each wizard (0 ≤ thrust ≤ 150, 0 ≤ power ≤ 500, 0 ≤ magic ≤ 1500)
            // i.e.: "MOVE x y thrust" or "THROW x y power" or "WINGARDIUM id x y magic"
            println!("MOVE 8000 3750 100");
            println!("{}", action)
        }
    }
}