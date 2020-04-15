use std::io;
use rand::{thread_rng, Rng};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

static BOUNDS: &'static [(i32, i32)] = &[(0, 0), (16001, 0), (0, 7501), (7501, 16001)];
static POLE_RAD: i32 = 300;
static SNAFFLE_RAD: i32 = 150;
static GOAL0_CENTER: (i32, i32) = (0, 3750);
static GOAL1_CENTER: (i32, i32) = (16000, 3750);
static GOAL0_BOUNDS: &'static [(i32, i32)] =
    &[(0, 3750 - 2000 + POLE_RAD + SNAFFLE_RAD),
        (0, 3750 + 2000 - POLE_RAD - SNAFFLE_RAD)];
static GOAL1_BOUNDS: &'static [(i32, i32)] =
    &[(16000, 3750 - 2000 + POLE_RAD + SNAFFLE_RAD),
        (16000, 3750 + 2000 - POLE_RAD - SNAFFLE_RAD)];
static MAX_THRUST: i32 = 150;
static MAX_POWER: i32 = 500;
static MAX_MAGIC: i32 = 100;
static MAX_MAGIC_POWER: i32 = 1500;


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

    pub fn destination(&self) -> (i32, i32) {}

    pub fn distance_from_goal(&self, team_id: i32) -> i32 {
        if team_id == 0 {
            (((self.x - GOAL1_CENTER[0]) as f32).powi(2) +
                ((self.y - GOAL1_CENTER[1]) as f32).powi(2))
                .sqrt() as i32
        } else {
            (((self.x - GOAL0_CENTER[0]) as f32).powi(2) +
                ((self.y - GOAL0_CENTER[1]) as f32).powi(2))
                .sqrt() as i32
        }
    }
}

struct Bludger {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
    last_wizard_hit: i32,
}

impl Bludger {
    pub fn new(id: i32, x: i32, y: i32, vx: i32, vy: i32, last_wizard_hit: i32) -> Bludger {
        Bludger { id, x, y, vx, vy, last_wizard_hit }
    }

    pub fn destination(&self) -> (i32, i32) {}

    pub fn last_wizard_hit(&self, wizards: &Vec<Wizard>) -> Option<Wizard> {
        match self.last_wizard_hit {
            -1 => None,
            _ => {
                for &w in wizards {
                    if w.id == self.last_wizard_hit {
                        return Some(w.clone());
                    }
                }
                None
            }
        }
    }

    pub fn set_last_wizard_hit(&mut self, wizard_id: i32) {
        self.last_wizard_hit = wizard_id;
    }

    pub fn set_coords_and_velocity(&mut self, x: i32, y: i32, vx: i32, vy: i32) {
        self.x = x;
        self.y = y;
        self.vx = vx;
        self.vy = vy;
    }

    pub fn collides(&self, wizards: &Vec<Wizard>) -> Option<Wizard> {}

    pub fn closest_target(&self, wizards: &Vec<Wizard>) -> Wizard {}
}

struct Wizard {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
    pub has_snaffle: bool,
    pub is_opponent: bool,
    pub last_hit: bool,
    pub team_id: i32,
}

impl Wizard {
    pub fn new(
        id: i32, x: i32, y: i32, vx: i32, vy: i32,
        has_sniffle: bool, is_opponent: bool, last_hit: bool, team_id: i32,
    ) -> Wizard {
        Wizard { id, x, y, vx, vy, has_snaffle: has_sniffle, is_opponent, last_hit, team_id }
    }

    pub fn destination(&self) -> (i32, i32) {}

    pub fn move_action(&self, snaffles: &Vec<Snaffle>) -> String {
        let snaffle = self.find_most_desirable_snaffle(snaffles);
        match snaffle {
            Some(s) => {
                let (x_to, y_to) = snaffle.destination();
                format!("{} {} {} {}", "MOVE", x_to, y_to, self.thrust_to_destination(x_to, y_to))
            }
            None => String::from("")
        }
    }
    pub fn throw_action(&self) -> String {
        let (x_to, y_to) = self.throw_destination();
        format!("{} {} {} {}", "THROW", x_to, y_to, self.power_to_destination(x_to, y_to))
    }

    pub fn should_magic(&self, magic: i32) -> bool { magic > 20 }

    pub fn magic_action(&self, wizards: &Vec<Wizard>, snaffles: &mut Vec<Snaffle>, &mut magic: i32, magic_use: i32) -> String {
        magic -= magic_use;
        let target = self.find_most_desirable_magic_target(wizards, snaffles);
        match target {
            Target::Wizard(w) => format!("{} {} {} {} {}", "WINGARDIUM", w.id, x_to, y_to, magic_use * 15),
            Target::Snaffle(s) => format!("{} {} {} {} {}", "WINGARDIUM", s.id, x_to, y_to, magic_use * 15)
        }
    }

    pub fn find_most_desirable_snaffle(&self, snaffles: &Vec<Snaffle>) -> Option<Snaffle> {
        let mut distance = 0.;
        let mut result = None;
        for snaffle in snaffles.iter() {
            let new_distance = (((snaffle.x - self.x) as f32).powi(2) + ((snaffle.y - self.y) as f32).powi(2));
            if new_distance < distance {
                distance = new_distance.clone();
                result = Some(snaffle.clone());
            }
        }
        result.into()
    }

    fn find_most_desirable_magic_target(&self, wizards: &Vec<Wizard>, snaffles: &mut Vec<Snaffle>) -> Target {
        snaffles.sort_by(|a, b|
            a.distance_from_goal(self.team_id.clone())
             .cmp(&b.distance_from_goal(self.team_id.clone())));
        match snaffles.first().cloned() {
            Some(s) => Target::Snaffle(s),
            None => Target::Wizard(wizards.first().cloned().unwrap())
        }
    }

    //ToDo don't use rand...
    fn throw_destination(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        if team_id == 0 {
            let x_to = GOAL1_BOUNDS[0][0];
            let y_to = rng.gen_range(GOAL1_BOUNDS[0][1], GOAL1_BOUNDS[1][1]);
        } else {
            let x_to = GOAL0_BOUNDS[0][0];
            let y_to = rng.gen_range(GOAL0_BOUNDS[0][1], GOAL0_BOUNDS[1][1]);
        }
        (x_to, y_to)
    }

    fn thrust_to_destination(&self, x_to: i32, y_to: i32) -> i32 {
        50
    }

    fn power_to_destination(&self, x_to: i32, y_to: i32) -> i32 {
        200
    }
}

enum Target {
    Snaffle(Snaffle),
    Wizard(Wizard),
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
    let has_snaffle = parse_input!(inputs[6], i32);
    (entity_id, entity_type, x, y, vx, vy, has_snaffle as bool)
}

/**
 * Grab Snaffles and try to throw them through the opponent's goal!
 * Move towards a Snaffle to grab it and use your team id to determine towards where you need to throw it.
 * Use the Wingardium spell to move things around at your leisure, the more magic you put it, the further they'll move.
 **/
fn main() {
    let my_team_id = parse_team_id();
    let mut game_started = false;
    //Holds state during whole game to keep track of last wizard hit
    let mut bludgers = vec![];

    // game loop
    loop {
        let (my_score, my_magic, opponent_score, opponent_magic, entities) = parse_loop_variables();
        let mut snaffles = vec![];
        let mut my_wizards = vec![];
        let mut opponent_wizards = vec![];
        let mut all_wizards = vec![];

        for i in 0..entities as usize {
            let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
            match &entity_type[..] {
                "WIZARD" => my_wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, false, false, my_team_id)),
                "OPPONENT_WIZARD" => opponent_wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, true, false, 1 - my_team_id)),
                "SNAFFLE" => snaffles.push(Snaffle::new(entity_id, x, y, vx, vy)),
                "BLUDGER" => {
                    if !game_started {
                        bludgers.push(Bludger::new(entity_id, x, y, vx, vy, -1));
                    } else {
                        for &mut bludger in bludgers {
                            if bludger.id == entity_id {
                                bludger.set_coords_and_velocity(x, y, vx, vy);
                            }
                        }
                    }
                }
                _ => ()
            }
            all_wizards = my_wizards.iter().cloned()
                                    .chain(opponent_wizards.iter().cloned())
                                    .collect();
        }
        for wizard in my_wizards.iter() {
            let action: String = if wizard.has_snaffle {
                wizard.throw_action()
            } else {
                if wizard.should_magic(magic) {
                    wizard.magic_action(&opponent_wizards, &snaffles, magic)
                } else { wizard.move_action(&snaffles) }
            };
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");


            // Edit this line to indicate the action for each wizard (0 ≤ thrust ≤ 150, 0 ≤ power ≤ 500, 0 ≤ magic ≤ 1500)
            // i.e.: "MOVE x y thrust" or "THROW x y power" or "WINGARDIUM id x y magic"
            println!("{}", action)
        }
        game_started = true;
    }
}