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

static BOUNDS: &'static [(i32, i32)] = &[(0, 0), (16001, 0), (0, 7501), (7501, 16001)];
static WIDTH: i32 = 16001;
static HEIGHT: i32 = 7501;
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Collider {
    pub pos: Vector2,
    pub vel: Vector2,
    pub friction: f32,
    pub mass: f32,
    pub radius: f32,
}

impl Collider {
    pub fn new(pos: Vector2, vel: Vector2, friction: f32, mass: f32, radius: f32) -> Collider {
        Collider { pos, vel, friction, mass, radius }
    }

    pub fn collides(&self, other: &Collider) -> bool {
        self.pos.distance(other.pos) < self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
enum EntityType {
    Snaffle,
    Wizard,
    Opponent,
    Bludger,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Entity {
    pub id: i32,
    pub entity_type: EntityType,
    pub collider: Collider,
    pub has_snaffle: bool,
}

impl Entity {
    pub fn new(id: i32, entity_type: EntityType, collider: Collider, has_snaffle: bool) -> Entity {
        Entity { id, entity_type, collider, has_snaffle }
    }

    pub fn update(&mut self, x: i32, y: i32, vx: i32, vy: i32, has_snaffle: bool) {
        self.collider.pos.x = x as f32;
        self.collider.pos.y = y as f32;
        self.collider.vel.x = vx as f32;
        self.collider.vel.x = vy as f32;
        self.has_snaffle = has_snaffle;
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Goal {
    pub pole_top: Collider,
    pub pole_bottom: Collider,
}

impl Goal {
    pub fn new(team_id: i32) -> Goal {
        if team_id == 0 {
            Goal {
                pole_top: Collider::new(
                    Vector2::new(16000.0, 3750.0 - 4000.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
                pole_bottom: Collider::new(
                    Vector2::new(16000.0, 3750.0 + 4000.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
            }
        } else {
            Goal {
                pole_top: Collider::new(
                    Vector2::new(0.0, 3750.0 - 4000.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
                pole_bottom: Collider::new(
                    Vector2::new(0.0, 3750.0 + 4000.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
            }
        }
    }

    pub fn inner_bounds(&self) -> (Vector2, Vector2) {
        (
            Vector2::new(self.pole_top.pos.x, self.pole_top.y + self.pole_top.radius),
            Vector2::new(self.pole_bottom.pos.x, self.pole_bottom.y - self.pole_top.radius),
        )
    }

    pub fn random_inside_goal(&self) -> Vector2 {
        let mut rng = thread_rng();
        let x_to = self.pole_top.x;
        let y_to = rng.gen_range(self.pole_top.pos.y, self.pole_bottom.pos.y);
        Vector2::new(x_to, y_to)
    }
}

enum ActionType {
    Throw,
    Move,
    Magic,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct State {
    pub entities: Vec<Entity>,
    pub magic: i32,
    pub team_id: i32,
    pub own_goal: Goal,
    pub target_goal: Goal,
}

impl State {
    pub fn new(team_id: i32) -> State {
        State {
            entities: vec![],
            magic: 0,
            team_id,
            own_goal: Goal::new(1 - team_id),
            target_goal: Goal::new(team_id),
        }
    }

    pub fn update(&mut self, init: bool) {
        let (_my_score, my_magic, _opponent_score, _opponent_magic, entities) = parse_loop_variables();
        self.magic = my_magic;
        if init {
            for _ in 0..entities as usize {
                let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
                match &entity_type[..] {
                    "WIZARD" => self.entities.push(
                        Entity::new(entity_id,
                                    EntityType::Wizard,
                                    Collider::new(
                                        Vector2::new(x as f32, y as f32),
                                        Vector2::new(vx as f32, vy as f32), 0.75, 1., 400.),
                                    has_snaffle)
                    ),
                    "OPPONENT_WIZARD" => self.entities.push(
                        Entity::new(entity_id,
                                    EntityType::Opponent,
                                    Collider::new(
                                        Vector2::new(x as f32, y as f32),
                                        Vector2::new(vx as f32, vy as f32), 0.75, 1., 400.),
                                    has_snaffle)
                    ),
                    "SNAFFLE" => self.entities.push(
                        Entity::new(entity_id,
                                    EntityType::Snaffle,
                                    Collider::new(
                                        Vector2::new(x as f32, y as f32),
                                        Vector2::new(vx as f32, vy as f32), 0.75, 0.5, 150.),
                                    has_snaffle)
                    ),
                    "BLUDGER" => self.entities.push(
                        Entity::new(entity_id,
                                    EntityType::Bludger,
                                    Collider::new(
                                        Vector2::new(x as f32, y as f32),
                                        Vector2::new(vx as f32, vy as f32), 0.9, 8., 200.),
                                    has_snaffle)
                    ),
                    _ => ()
                }
            }
        } else {
            let mut existing_snaffles = vec![];
            for _ in 0..entities as usize {
                let (entity_id, entity_type, x, y, vx, vy, has_snaffle) = parse_entity_variables();
                match &entity_type[..] {
                    "SNAFFLE" => existing_snaffles.push(
                        Entity::new(entity_id,
                                    EntityType::Snaffle,
                                    Collider::new(
                                        Vector2::new(x as f32, y as f32),
                                        Vector2::new(vx as f32, vy as f32), 0.75, 0.5, 150.),
                                    has_snaffle)),
                    _ => self.entities.iter_mut().find(|e| e.id == entity_id).unwrap().update(x, y, vx, vy, has_snaffle)
                }
            }
            let entities_to_remove = self.entities.iter()
                                         .filter(|e| {
                                             e.entity_type == EntityType::Snaffle &&
                                                 existing_snaffles.iter().all(|s| s.id != e.id)
                                         }).map(|e| e.id).collect::<Vec<i32>>();
            let new_entities = self.entities.clone().iter()
                                   .filter(|e1| {
                                       entities_to_remove.iter().all(|e2| e1.id != e2.id)
                                   }).collect::<Vec<Entity>>();
            self.entities = new_entities;
        }
    }

    pub fn act_turn(&mut self) {
        let mut magic_left = self.magic;
        for wizard in &self.wizards() {
            match self.optimal_action(&wizard, magic_left) {
                ActionType::Throw => {
                    let dest: Vector2 = self.throw_destination(&wizard);
                    self.throw_action(
                        &dest,
                        self.throw_power(&dest),
                    );
                }
                ActionType::Magic => {
                    let target: Entity = self.magic_target(&wizard);
                    let dest: Vector2 = self.magic_destination(&wizard);
                    let magic_power = self.magic_power(&target.pos, &dest, &magic_left);
                    self.magic_action(
                        target.id,
                        &dest,
                        magic_power,
                    );
                    magic_left -= magic_power;
                }
                ActionType::Move => {
                    let dest: Vector2 = self.move_destination(&wizard);
                    self.move_action(
                        &dest,
                        self.move_thrust(&wizard, &dest),
                    )
                }
            }
        }
    }

    fn other_wizard(&self, wizard: &Entity) -> Entity {
        self.wizards().iter().find(|e| e.id != wizard.id).cloned().unwrap()
    }

    fn move_action(&self, dest: &Vector2, thrust: i32) {
        println!("{} {} {} {}", "MOVE", dest.x as i32, dest.y as i32, thrust)
    }

    fn throw_action(&self, dest: &Vector2, power: i32) {
        println!("{} {} {} {}", "THROW", dest.x as i32, dest.y as i32, power)
    }

    fn magic_action(&mut self, target_id: i32, dest: &Vector2, magic_power: i32) {
        println!("{} {} {} {} {}", "WINGARDIUM", target_id, dest.x as i32, dest.y as i32, magic_power)
    }

    fn entities_of_type(&self, entity_type: EntityType) -> Vec<Entity> {
        self.entities.iter()
            .filter(|e| e.entity_type == entity_type).cloned().collect()
    }

    fn wizards(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Wizard) }

    fn opponents(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Opponent) }

    fn bludgers(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Bludger) }

    fn snaffles(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Snaffle) }
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