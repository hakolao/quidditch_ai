use std::io;
use rand::{thread_rng, Rng};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
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

// static BOUNDS: &'static [(i32, i32)] = &[(0, 0), (16001, 0), (0, 7501), (7501, 16001)];
static WIDTH: i32 = 16001;
static HEIGHT: i32 = 7501;
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

    pub fn average(vectors: Vec<Vector2>) -> Vector2 {
        let mut sum = Vector2::new(0., 0.);
        for v in &vectors {
            sum = sum.add(v.clone());
        }
        sum.div_num(vectors.len() as f32)
    }

    pub fn div_num(&self, num: f32) -> Vector2 {
        let mut num = num.clone();
        if num == 0. { num += 0.0001 }
        Vector2::new(self.x / num, self.y / num)
    }

    pub fn div(&self, v2: Vector2) -> Vector2 {
        let mut v2 = v2.clone();
        if v2.x == 0. { v2.x += 0.0001 }
        if v2.y == 0. { v2.y += 0.0001 }
        Vector2::new(
            self.x / v2.x,
            self.y / v2.y,
        )
    }

    pub fn center_of_mass(&self, vecs: Vec<Vector2>) -> Vector2 {
        Vector2::new(
            vecs.iter().map(|v| v.x).sum::<f32>() / (vecs.len() as f32),
            vecs.iter().map(|v| v.y).sum::<f32>() / (vecs.len() as f32))
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
            team_id,
            friction: 0.75,
            radius: 400.0,
        }
    }

    pub fn update(&mut self, x: i32, y: i32, vx: i32, vy: i32, has_snaffle: bool) {
        self.pos = Vector2::new(x as f32, y as f32);
        self.velocity = Vector2::new(vx as f32, vy as f32);
        self.has_snaffle = has_snaffle;
    }

    pub fn set_target(&mut self, target: Option<Snaffle>) {
        self.target_snaffle = target;
    }

    #[allow(dead_code)]
    fn destination(&self) -> Vector2 {
        self.pos.add(self.velocity.mul_num(self.friction))
    }
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

    pub fn update(&mut self, init: bool) {
        let (_my_score, my_magic, _opponent_score, _opponent_magic, entities) = parse_loop_variables();
        self.magic = my_magic;
        let mut snaffles = vec![];
        if init {
            for _ in 0..entities as usize {
                let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
                match &entity_type[..] {
                    "WIZARD" => self.wizards.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, self.team_id)),
                    "OPPONENT_WIZARD" => self.opponents.push(Wizard::new(entity_id, x, y, vx, vy, has_snaffle, 1 - self.team_id)),
                    "SNAFFLE" => snaffles.push(Snaffle::new(entity_id, x, y, vx, vy)),
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
                    // Snaffles may be removed from game so just replace with new vector
                    "SNAFFLE" => snaffles.push(Snaffle::new(entity_id, x, y, vx, vy)),
                    "BLUDGER" => self.bludgers.iter_mut().find(|b| b.id == entity_id).unwrap().update(x, y, vx, vy),
                    _ => ()
                }
            }
        }
        self.snaffles = snaffles;
    }

    pub fn act_turn(&mut self) {
        let mut magic_used = false;
        self.set_most_optimal_targets();
        for wizard in &self.wizards {
            if wizard.has_snaffle {
                let dest = self.optimal_throw_destination();
                self.throw_action(dest, self.power_to_destination(wizard, dest));
            } else if !magic_used && self.should_magic() {
                let target = self.optimal_magic_target();
                match target {
                    Some(t) => {
                        let dest = self.optimal_magic_destination(&t);
                        self.magic_action(t.id, dest, self.magic)
                    }
                    None => {
                        let target = self.opponents.first().unwrap().clone();
                        let dest = Vector2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
                        self.magic_action(target.id, dest, self.magic)
                    }
                }
                magic_used = true;
            } else {
                match wizard.target_snaffle {
                    Some(snaffle) => self.move_action(snaffle.pos, self.thrust_to_destination(wizard, snaffle.pos)),
                    None => {
                        //Wizard should not be without target after self.set_most_optimal_targets();
                        let center = Vector2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
                        self.move_action(center, self.thrust_to_destination(wizard, center))
                    }
                }
            }
        }
    }

    fn move_action(&self, destination: Vector2, thrust: i32) {
        println!("{} {} {} {}", "MOVE", destination.x as i32, destination.y as i32, thrust)
    }

    fn throw_action(&self, destination: Vector2, power: i32) {
        println!("{} {} {} {}", "THROW", destination.x as i32, destination.y as i32, power)
    }

    fn magic_action(&self, target_id: i32, destination: Vector2, magic_to_use: i32) {
        println!("{} {} {} {} {}", "WINGARDIUM", target_id, destination.x as i32, destination.y as i32, magic_to_use)
    }

    fn optimal_throw_destination(&self) -> Vector2 {
        //ToDo Improve!!
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

    fn optimal_magic_target(&self) -> Option<Snaffle> {
        let mut snaffles = self.snaffles.clone();
        snaffles.sort_by(|a, b| {
            (a.distance_from_goal(self.team_id) as i32).cmp(
                &(b.distance_from_goal(self.team_id) as i32))
        });
        // last() for closest to own goal
        // first() for closest to opponent goal
        snaffles.last().cloned()
    }

    fn optimal_magic_destination(&self, _target: &Snaffle) -> Vector2 {
        //ToDo Improve!!
        self.optimal_throw_destination()
    }

    fn set_most_optimal_targets(&mut self) {
        let wizards = self.wizards.clone();
        let mut new_wizards = vec![];
        // Set targets to None if target was removed
        for w in &wizards {
            let mut wiz = w.clone();
            self.validate_wizard_target(&mut wiz);
            new_wizards.push(wiz);
        }
        let mut wizard1 = new_wizards[0];
        let mut wizard2 = new_wizards[1];
        wizard1.set_target(self.most_desirable_target(&wizard1, &wizard2));
        wizard2.set_target(self.most_desirable_target(&wizard2, &wizard1));
        self.wizards[0] = wizard1;
        self.wizards[1] = wizard2;
    }

    fn validate_wizard_target(&mut self, wizard: &mut Wizard) {
        if wizard.target_snaffle.is_some() &&
            self.snaffles.iter().all(|s| s.id != wizard.target_snaffle.unwrap().id) {
            wizard.set_target(None);
        }
    }

    fn most_desirable_target(&self, wizard: &Wizard, other_wizard: &Wizard) -> Option<Snaffle> {
        let mut snaffles = self.snaffles.clone();
        if snaffles.len() == 0 { return None; }
        if snaffles.len() > 1 {
            if other_wizard.target_snaffle.is_some() {
                snaffles.remove(snaffles.iter().position(|s|
                    s.id == other_wizard.target_snaffle.unwrap().id
                ).unwrap());
            }
        }
        // Calculate desirability
        let mut snaffle_desirabilities: Vec<(Snaffle, i32)> = snaffles.iter().map(|s| {
            let mut desirability = 0;
            //Any opponent collides with snaffle
            if self.opponents.iter().any(|o| s.collides_with_wizard(o)) {
                desirability -= 10;
            }
            //Snaffle is close
            if s.pos.distance(wizard.pos) < 3000. {
                desirability += 10;
            }
            //All opponents further than wizard from snaffle
            if self.opponents.iter().all(|o| s.pos.distance(o.pos) > s.pos.distance(wizard.pos)) {
                desirability += 15;
            }
            //Any opponents further than wizard from snaffle
            else if self.opponents.iter().any(|o| s.pos.distance(o.pos) > s.pos.distance(wizard.pos)) {
                desirability += 5;
            }
            (s.clone(), desirability)
        }).collect();
        snaffle_desirabilities.sort_by(|a, b| a.1.cmp(&b.1));
        Some(snaffle_desirabilities.last().unwrap().0)
    }

    fn should_magic(&self) -> bool {
        self.magic > MAX_MAGIC / 3
    }

    fn free_opponents(&self) -> Vec<Wizard> {
        self.opponents.iter().filter(|o| o.has_snaffle == false).cloned().collect()
    }

    fn taken_opponents(&self) -> Vec<Wizard> {
        self.opponents.iter().filter(|o| o.has_snaffle == true).cloned().collect()
    }

    fn taken_snaffles(&self) -> Vec<Snaffle> {
        let taken_opponents = self.taken_opponents();
        if taken_opponents.len() > 0 {
            return self.snaffles.iter().filter(|s|
                taken_opponents.iter().any(|o| s.collides_with_wizard(o))
            ).cloned().collect();
        }
        vec![]
    }

    fn free_snaffles(&self) -> Vec<Snaffle> {
        let taken_snaffles = self.taken_snaffles();
        self.snaffles.iter().filter(|&s1|
            !taken_snaffles.iter().any(|s2| s2 == s1)).cloned().collect()
    }

    fn thrust_to_destination(&self, wizard: &Wizard, destination: Vector2) -> i32 {
        let dist = wizard.pos.distance(destination) as i32;
        if dist > MAX_THRUST
        { MAX_THRUST } else { dist }
    }

    fn power_to_destination(&self, wizard: &Wizard, destination: Vector2) -> i32 {
        let dist = wizard.pos.distance(destination) as i32;
        if dist > MAX_POWER
        { MAX_POWER } else { dist }
    }
}

fn main() {
    let my_team_id = parse_team_id();
    let mut init = true;
    let mut state = State::new(my_team_id);

    loop {
        state.update(init);
        state.act_turn();
        init = false;
    }
}