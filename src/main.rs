use std::io;
use rand::{thread_rng, Rng};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// static BOUNDS: &'static [(i32, i32)] = &[(0, 0), (16001, 0), (0, 7501), (7501, 16001)];
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
static MAGIC_MUL: i32 = 15;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[allow(dead_code)]
    pub fn normalized(&self) -> Vector2 {
        let len = self.len();
        Vector2::new(self.x / len, self.y / len)
    }

    pub fn add(&self, v2: Vector2) -> Vector2 {
        Vector2::new(self.x + v2.x, self.y + v2.y)
    }

    pub fn distance(&self, v2: Vector2) -> f32 {
        ((self.x - v2.x).powi(2) +
            (self.y - v2.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Snaffle {
    pub id: i32,
    pub pos: Vector2,
    pub velocity: Vector2,
    friction: f32,
    radius: f32,
}

impl Snaffle {
    pub fn new(id: i32, x: i32, y: i32, vx: i32, vy: i32) -> Snaffle {
        Snaffle {
            id,
            pos: Vector2::new(x as f32, y as f32),
            velocity: Vector2::new(vx as f32, vy as f32),
            friction: 0.75,
            radius: 150.0,
        }
    }

    pub fn destination(&self) -> Vector2 {
        self.pos.add(self.velocity)
    }

    pub fn distance_from_goal(&self, team_id: i32) -> f32 {
        let goal_center: Vector2 = if team_id == 0 {
            Vector2::new(GOAL1_CENTER.0 as f32, GOAL1_CENTER.1 as f32)
        } else { Vector2::new(GOAL0_CENTER.0 as f32, GOAL0_CENTER.1 as f32) };
        goal_center.distance(self.pos)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Bludger {
    pub id: i32,
    pub pos: Vector2,
    pub velocity: Vector2,
    last_wizard_hit: i32,
    friction: f32,
    radius: f32,
}

impl Bludger {
    pub fn new(id: i32, x: i32, y: i32, vx: i32, vy: i32, last_wizard_hit: i32) -> Bludger {
        Bludger {
            id,
            pos: Vector2::new(x as f32, y as f32),
            velocity: Vector2::new(vx as f32, vy as f32),
            last_wizard_hit,
            friction: 0.9,
            radius: 200.0,
        }
    }

    #[allow(dead_code)]
    pub fn destination(&self) -> Vector2 {
        self.pos.add(self.velocity)
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn set_last_wizard_hit(&mut self, wizard_id: i32) {
        self.last_wizard_hit = wizard_id;
    }

    pub fn set_coords_and_velocity(&mut self, x: i32, y: i32, vx: i32, vy: i32) {
        self.pos.x = x as f32;
        self.pos.y = y as f32;
        self.velocity.x = vx as f32;
        self.velocity.x = vy as f32;
    }

    pub fn collides(&self, wizard: &Wizard) -> bool {
        wizard.pos.distance(self.pos) < wizard.radius + self.radius
    }

    #[allow(dead_code)]
    pub fn closest_target(&self, wizards: &Vec<Wizard>) -> Wizard {
        wizards.iter().fold(wizards[0].clone(), |mut result, wizard| {
            if wizard.pos.distance(self.pos) < result.pos.distance(self.pos)
                && wizard.last_hit == false {
                result = wizard.clone();
            }
            result
        })
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Wizard {
    pub id: i32,
    pub pos: Vector2,
    pub velocity: Vector2,
    pub has_snaffle: bool,
    pub is_opponent: bool,
    pub last_hit: bool,
    pub team_id: i32,
    pub target_snaffle: Option<Snaffle>,
    friction: f32,
    radius: f32,
}

impl Wizard {
    pub fn new(
        id: i32, x: i32, y: i32, vx: i32, vy: i32,
        has_sniffle: bool, is_opponent: bool, last_hit: bool, team_id: i32,
    ) -> Wizard {
        Wizard {
            id,
            pos: Vector2::new(x as f32, y as f32),
            velocity: Vector2::new(vx as f32, vy as f32),
            has_snaffle: has_sniffle,
            is_opponent,
            last_hit,
            team_id,
            target_snaffle: None,
            friction: 0.75,
            radius: 400.0,
        }
    }

    #[allow(dead_code)]
    pub fn destination(&self) -> Vector2 {
        self.pos.add(self.velocity)
    }

    pub fn move_action(&mut self, other_wizard: &mut Wizard, snaffles: &Vec<Snaffle>) -> String {
        let snaffle = self.find_most_desirable_snaffle(other_wizard, snaffles);
        self.target_snaffle = snaffle;
        match self.target_snaffle {
            Some(s) => {
                let dest = s.destination();
                format!("{} {} {} {}", "MOVE", dest.x, dest.y, self.thrust_to_destination(dest))
            }
            None => format!("{} {} {} {}", "MOVE", 0, 0, 0)
        }
    }
    pub fn throw_action(&self) -> String {
        let dest = self.throw_destination();
        format!("{} {} {} {}", "THROW", dest.x, dest.y, self.power_to_destination(dest))
    }

    pub fn should_magic(&self, snaffles: &Vec<Snaffle>, magic: i32) -> bool {
        // if any snaffle is close to own goal (get it away from there! :D)
        magic > (0.3 * MAX_MAGIC as f32) as i32 ||
            snaffles.iter().any(|s| s.distance_from_goal(1 - self.team_id) <= (magic * MAGIC_MUL) as f32)
    }

    pub fn magic_action(&self, wizards: &Vec<Wizard>, snaffles: &Vec<Snaffle>, magic: i32) -> String {
        let target = self.find_most_desirable_magic_target(wizards, snaffles);
        // Goal (Away for wizard targets)
        let dest = self.throw_destination();
        match target {
            Target::Wizard(w) => format!("{} {} {} {} {}", "WINGARDIUM", w.id, dest.x, dest.y, magic),
            Target::Snaffle(s) => format!("{} {} {} {} {}", "WINGARDIUM", s.id, dest.x, dest.y, magic)
        }
    }

    // If another wizard has target, ignore that target from snaffles list.
    // Select snaffle closest to opponent goal
    pub fn find_most_desirable_snaffle(&self, other_wizard: &mut Wizard, snaffles: &Vec<Snaffle>) -> Option<Snaffle> {
        let mut snaffles = snaffles.clone();

        if other_wizard.target_snaffle.is_some() {
            snaffles.remove(
                snaffles.iter().position(|s| { s.id == other_wizard.target_snaffle.unwrap().id }).unwrap()
            );
        }

        snaffles.sort_by(|a, b|
            (a.distance_from_goal(self.team_id) as i32)
                .cmp(&(b.distance_from_goal(self.team_id) as i32))
        );
        snaffles.first().cloned()
    }

    fn find_most_desirable_magic_target(&self, opponents: &Vec<Wizard>, snaffles: &Vec<Snaffle>) -> Target {
        let mut ops = opponents.clone();
        let mut snaffles = snaffles.clone();

        // Sort snaffles by closest to goal
        snaffles.sort_by(|a, b|
            (a.distance_from_goal(self.team_id) as i32)
                .cmp(&(b.distance_from_goal(self.team_id) as i32))
        );
        // But select closest to own goal!
        let snaffle = snaffles.last().cloned();
        // Sort opponents by closest to a snaffle
        ops.sort_by(|a, b|
            snaffles.iter().map(|s| s.pos.distance(a.pos) as i32).min()
                    .cmp(&snaffles.iter().map(|s| s.pos.distance(b.pos) as i32).min()));
        let op_wizard = ops.first().cloned().unwrap();
        match snaffle {
            Some(s) => {
                // Distance from own goal for snaffle less than op distance from snaffle
                if s.distance_from_goal(1 - self.team_id) < op_wizard.pos.distance(s.pos)
                { Target::Snaffle(s) } else { Target::Wizard(op_wizard) }
            }
            None => Target::Wizard(op_wizard)
        }
    }

    //ToDo don't use rand...
    fn throw_destination(&self) -> Vector2 {
        let mut rng = thread_rng();
        if self.team_id == 0 {
            let x_to = GOAL1_BOUNDS[0].0;
            let y_to = rng.gen_range(GOAL1_BOUNDS[0].1, GOAL1_BOUNDS[1].1);
            Vector2::new(x_to as f32, y_to as f32)
        } else {
            let x_to = GOAL0_BOUNDS[0].0;
            let y_to = rng.gen_range(GOAL0_BOUNDS[0].1, GOAL0_BOUNDS[1].1);
            Vector2::new(x_to as f32, y_to as f32)
        }
    }

    fn thrust_to_destination(&self, destination: Vector2) -> i32 {
        let dist = self.pos.distance(destination) as i32;
        if dist > MAX_THRUST
        { MAX_THRUST } else { dist }
    }

    fn power_to_destination(&self, destination: Vector2) -> i32 {
        let dist = self.pos.distance(destination) as i32;
        if dist > MAX_POWER
        { MAX_POWER } else { dist }
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
    (entity_id, entity_type, x, y, vx, vy, has_snaffle == 1)
}

fn main() {
    let my_team_id = parse_team_id();
    let mut game_started = false;
    //Holds state during whole game to keep track of last wizard hit
    let mut bludgers = vec![];

    loop {
        let (_my_score, my_magic, _opponent_score, _opponent_magic, entities) = parse_loop_variables();
        let mut snaffles = vec![];
        let mut my_wizards = vec![];
        let mut opponent_wizards = vec![];
        let mut _all_wizards = vec![];

        for _ in 0..entities as usize {
            let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
            match &entity_type[..] {
                "WIZARD" => my_wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, false, false, my_team_id)),
                "OPPONENT_WIZARD" => opponent_wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, true, false, 1 - my_team_id)),
                "SNAFFLE" => snaffles.push(Snaffle::new(entity_id, x, y, vx, vy)),
                "BLUDGER" => {
                    if !game_started {
                        bludgers.push(Bludger::new(entity_id, x, y, vx, vy, -1));
                    } else {
                        for bludger in bludgers.iter_mut() {
                            if bludger.id == entity_id {
                                bludger.set_coords_and_velocity(x, y, vx, vy);
                            }
                        }
                    }
                }
                _ => ()
            }
            _all_wizards = my_wizards.iter().cloned()
                                     .chain(opponent_wizards.iter().cloned())
                                     .collect();
        }

        let mut magic_used = false;
        for (index, wizard) in my_wizards.iter_mut().enumerate() {
            for b in bludgers.clone() {
                if b.collides(wizard) { wizard.last_hit = true }
            }
            let action: String = if wizard.has_snaffle {
                wizard.throw_action()
            } else {
                if !magic_used && wizard.should_magic(&snaffles, my_magic) {
                    magic_used = true;
                    wizard.magic_action(&opponent_wizards, &snaffles, my_magic)
                } else { wizard.move_action(&mut my_wizards[my_wizards.len() - (index + 1)].clone(), &snaffles) }
            };
            println!("{}", action)
        }
        game_started = true;
    }
}