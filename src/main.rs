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

    pub fn mul_num(&self, num: f32) -> Vector2 {
        Vector2::new(self.x * num, self.y * num)
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

    pub fn update(&mut self, x: i32, y: i32, vx: i32, vy: i32) {
        self.pos.x = x as f32;
        self.pos.y = y as f32;
        self.velocity.x = vx as f32;
        self.velocity.x = vy as f32;
        //ToDo Set last wizard hit or something...
    }

    pub fn destination(&self) -> Vector2 {
        self.pos.add(self.velocity.mul_num(self.friction))
    }

    pub fn distance_from_goal(&self, team_id: i32) -> f32 {
        let goal_center: Vector2 = if team_id == 0 {
            Vector2::new(GOAL1_CENTER.0 as f32, GOAL1_CENTER.1 as f32)
        } else { Vector2::new(GOAL0_CENTER.0 as f32, GOAL0_CENTER.1 as f32) };
        goal_center.distance(self.pos)
    }

    pub fn collides_with_wizard(&self, wizard: &Wizard) -> bool {
        wizard.pos.distance(self.pos) < wizard.radius + self.radius
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

    pub fn update(&mut self, x: i32, y: i32, vx: i32, vy: i32) {
        self.pos.x = x as f32;
        self.pos.y = y as f32;
        self.velocity.x = vx as f32;
        self.velocity.x = vy as f32;
    }

    #[allow(dead_code)]
    pub fn destination(&self) -> Vector2 {
        self.pos.add(self.velocity.mul_num(self.friction))
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

    pub fn collides_with_wizard(&self, wizard: &Wizard) -> bool {
        wizard.pos.distance(self.pos) < wizard.radius + self.radius
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Wizard {
    pub id: i32,
    pub pos: Vector2,
    pub velocity: Vector2,
    pub has_snaffle: bool,
    pub target_snaffle: Option<Snaffle>,
    pub should_magic: bool,
    team_id: i32,
    friction: f32,
    radius: f32,
}

impl Wizard {
    pub fn new(
        id: i32, x: i32, y: i32, vx: i32, vy: i32,
        has_snaffle: bool, team_id: i32,
    ) -> Wizard {
        Wizard {
            id,
            pos: Vector2::new(x as f32, y as f32),
            velocity: Vector2::new(vx as f32, vy as f32),
            has_snaffle,
            target_snaffle: None,
            should_magic: false,
            team_id,
            friction: 0.75,
            radius: 400.0,
        }
    }

    pub fn act(&mut self, state: &mut State) {
        let action: String = if self.has_snaffle {
            self.throw_action()
        } else {
            if self.should_magic {
                self.magic_action(state)
            } else {
                self.move_action()
            }
        };
        println!("{}", action)
    }

    pub fn update(&mut self, x: i32, y: i32, vx: i32, vy: i32, has_snaffle: bool) {
        self.pos = Vector2::new(x as f32, y as f32);
        self.velocity = Vector2::new(vx as f32, vy as f32);
        self.has_snaffle = has_snaffle;
    }

    pub fn decision_make(&mut self, state: &State) {
        self.target_snaffle = self.find_most_desirable_snaffle(state);
        self.should_magic = self.should_magic(state);
    }

    #[allow(dead_code)]
    fn destination(&self) -> Vector2 {
        self.pos.add(self.velocity.mul_num(self.friction))
    }

    fn move_action(&self) -> String {
        match self.target_snaffle {
            Some(s) => {
                let dest = s.destination();
                format!("{} {} {} {}", "MOVE", dest.x as i32, dest.y as i32, self.thrust_to_destination(dest))
            }
            None => format!("{} {} {} {}", "MOVE", 0, 0, 0)
        }
    }
    fn throw_action(&self) -> String {
        let dest = self.throw_destination();
        format!("{} {} {} {}", "THROW", dest.x as i32, dest.y as i32, self.power_to_destination(dest))
    }

    fn should_magic(&self, state: &State) -> bool {
        // Strategy to target snaffles when opponents have a snaffle
        state.magic > 10 && state.opponents.iter().any(|o| o.has_snaffle)
    }

    fn magic_action(&self, state: &mut State) -> String {
        let target = match self.find_most_desirable_magic_target(state) {
            Some(t) => t,
            None => Target::Wizard(state.opponents.first().cloned().unwrap())
        };
        let dest = self.throw_destination();
        let magic_to_use = state.magic / 2;
        state.magic -= magic_to_use;
        match target {
            Target::Wizard(w) =>
                format!("{} {} {} {} {}", "WINGARDIUM", w.id, dest.x as i32, dest.y as i32, magic_to_use),
            Target::Snaffle(s) =>
                format!("{} {} {} {} {}", "WINGARDIUM", s.id, dest.x as i32, dest.y as i32, magic_to_use)
        }
    }

    fn find_most_desirable_snaffle(&self, state: &State) -> Option<Snaffle> {
        let mut snaffles = state.free_snaffles();
        // Choose closest snaffle
        snaffles.sort_by(|a, b|
            (a.pos.distance(self.pos) as i32)
                .cmp(&(b.pos.distance(self.pos) as i32))
        );
        snaffles.first().cloned()
    }

    // Snaffles are most desirable?
    fn find_most_desirable_magic_target(&self, state: &State) -> Option<Target> {
        let mut snaffles = state.free_snaffles();
        if snaffles.len() == 0 {
            return None;
        }
        // Choose farthest snaffle
        snaffles.sort_by(|a, b|
            (a.pos.distance(self.pos) as i32)
                .cmp(&(b.pos.distance(self.pos) as i32))
        );
        Some(Target::Snaffle(snaffles.last().cloned().unwrap()))
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct State {
    pub snaffles: Vec<Snaffle>,
    pub opponents: Vec<Wizard>,
    pub wizards: Vec<Wizard>,
    pub bludgers: Vec<Bludger>,
    pub magic: i32,
    pub team_id: i32,
}

impl State {
    pub fn new(team_id: i32) -> State {
        State {
            snaffles: vec![],
            opponents: vec![],
            wizards: vec![],
            bludgers: vec![],
            magic: 0,
            team_id,
        }
    }

    pub fn other_wizard(&self, id: i32) -> Wizard {
        self.wizards.iter().find(|w| w.id != id).cloned().unwrap()
    }

    pub fn free_opponents(&self) -> Vec<Wizard> {
        self.opponents.iter().filter(|o| o.has_snaffle == false).cloned().collect()
    }

    pub fn taken_opponents(&self) -> Vec<Wizard> {
        self.opponents.iter().filter(|o| o.has_snaffle == true).cloned().collect()
    }

    pub fn taken_snaffles(&self) -> Vec<Snaffle> {
        let taken_opponents = self.taken_opponents();
        if taken_opponents.len() > 0 {
            return self.snaffles.iter().filter(|s|
                taken_opponents.iter().any(|o| s.collides_with_wizard(o))
            ).cloned().collect();
        }
        vec![]
    }

    pub fn free_snaffles(&self) -> Vec<Snaffle> {
        let taken_snaffles = self.taken_snaffles();
        self.snaffles.iter().filter(|&s1|
            !taken_snaffles.iter().any(|s2| s2 == s1)).cloned().collect()
    }

    pub fn update_state(&mut self, init: bool) {
        let (_my_score, my_magic, _opponent_score, _opponent_magic, entities) = parse_loop_variables();

        if init {
            for _ in 0..entities as usize {
                let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
                match &entity_type[..] {
                    "WIZARD" => self.wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, self.team_id)),
                    "OPPONENT_WIZARD" => self.opponents.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, 1 - self.team_id)),
                    "SNAFFLE" => self.snaffles.push(Snaffle::new(entity_id, x, y, vx, vy)),
                    "BLUDGER" => self.bludgers.push(Bludger::new(entity_id, x, y, vx, vy, -1)),
                    _ => ()
                }
            }
        } else {
            for _ in 0..entities as usize {
                let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
                match &entity_type[..] {
                    "WIZARD" => self.wizards.iter_mut().find(|w| w.id == entity_id).unwrap().update(x, y, vx, vy, has_snaffle),
                    "OPPONENT_WIZARD" => self.opponents.iter_mut().find(|w| w.id == entity_id).unwrap().update(x, y, vx, vy, has_snaffle),
                    "SNAFFLE" => self.snaffles.iter_mut().find(|s| s.id == entity_id).unwrap()
                                     .update(x, y, vx, vy),
                    "BLUDGER" => self.bludgers.iter_mut().find(|b| b.id == entity_id).unwrap()
                                     .update(x, y, vx, vy),
                    _ => ()
                }
            }
        }
    }
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
    let mut init = true;
    let mut state = State::new(my_team_id);

    loop {
        state.update_state(!init);

        let mut wizard1 = state.wizards[0];
        let mut wizard2 = state.wizards[1];
        wizard1.decision_make(&state);
        wizard2.decision_make(&state);

        wizard1.act(&mut state);
        wizard2.act(&mut state);
        
        init = false;
    }
}