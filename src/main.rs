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
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn normalized(&self) -> Vector2 {
        let len = self.len();
        Vector2::new(self.x / len, self.y / len)
    }
    pub fn add(&self, v2: Vector2) -> Vector2 {
        Vector2::new(self.x + v2.x, self.y + v2.y)
    }
    pub fn heading(&self, target: Vector2) -> Vector2 { Vector2::new(target.x - self.x, target.y - self.y) }
    pub fn direction(&self, target: Vector2) -> Vector2 {
        let heading = self.heading(target);
        let dist = self.distance(target);
        Vector2::new(heading.x / dist, heading.y / dist)
    }
    pub fn mul_num(&self, num: f32) -> Vector2 {
        Vector2::new(self.x * num, self.y * num)
    }
    pub fn mul(&self, v2: Vector2) -> Vector2 { Vector2::new(self.x * v2.x, self.y * v2.y) }
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
    pub fn destination(&self) -> Vector2 {
        self.pos.add(self.vel.mul_num(self.friction))
    }
    //ToDo: Implement boundary checks, bounces & collisions
    pub fn destination_turns(&self, turns: i32) -> Vector2 {
        self.pos.add(self.vel.mul_num(self.friction.powi(turns)))
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum EntityType {
    Snaffle,
    Wizard,
    Opponent,
    Bludger,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Entity {
    pub id: i32,
    pub entity_type: EntityType,
    pub collider: Collider,
    pub has_snaffle: bool,
    pub target: Option<i32>,
}

impl Entity {
    pub fn new(id: i32, entity_type: EntityType, collider: Collider, has_snaffle: bool) -> Entity {
        Entity { id, entity_type, collider, has_snaffle, target: None }
    }
    pub fn update(&mut self, x: i32, y: i32, vx: i32, vy: i32, has_snaffle: bool) {
        self.collider.pos.x = x as f32;
        self.collider.pos.y = y as f32;
        self.collider.vel.x = vx as f32;
        self.collider.vel.x = vy as f32;
        self.has_snaffle = has_snaffle;
    }
    pub fn set_target(&mut self, target: Option<i32>) {
        self.target = target;
    }
    //ToDo could implement destinations & values per turn, e.g future(2) 2 turns onwards
    pub fn future(&self) -> Entity {
        Entity {
            id: self.id,
            entity_type: self.entity_type.clone(),
            collider: Collider::new(
                self.collider.destination(),
                self.collider.vel.mul_num(self.collider.friction),
                self.collider.friction,
                self.collider.mass,
                self.collider.radius,
            ),
            has_snaffle: self.has_snaffle,
            target: self.target.clone(),
        }
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
    pub fn destination_is_close(&self, entity: &Entity, close_to_limit: f32) -> bool {
        self.points_inside_goal().iter().any(|&point| {
            let dist_from_point = entity.collider.destination().distance(point);
            dist_from_point < close_to_limit
        })
    }
    pub fn points_inside_goal(&self) -> Vec<Vector2> {
        let div = 6.;
        let mut points = vec![];
        for i in 0..(div as i32) {
            points.push(Vector2::new(
                self.pole_bottom.pos.x,
                self.pole_top.pos.y + i as f32 * (4000.0 / div),
            ))
        }
        points
    }
    pub fn inner_bounds(&self) -> (Vector2, Vector2) {
        (
            Vector2::new(self.pole_top.pos.x, self.pole_top.pos.y + self.pole_top.radius),
            Vector2::new(self.pole_bottom.pos.x, self.pole_bottom.pos.y - self.pole_top.radius),
        )
    }
    pub fn center(&self) -> Vector2 { Vector2::new(self.pole_bottom.pos.x, 3750.0) }
    pub fn random_inside_goal(&self) -> Vector2 {
        let mut rng = thread_rng();
        let x_to = self.pole_top.pos.x;
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
    entities: Vec<Entity>,
    magic: i32,
    team_id: i32,
    own_goal: Goal,
    target_goal: Goal,
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
                    "SNAFFLE" => {
                        existing_snaffles.push(
                            Entity::new(entity_id,
                                        EntityType::Snaffle,
                                        Collider::new(
                                            Vector2::new(x as f32, y as f32),
                                            Vector2::new(vx as f32, vy as f32), 0.75, 0.5, 150.),
                                        has_snaffle));
                        self.entities.iter_mut().find(|e| e.id == entity_id).unwrap().update(x, y, vx, vy, has_snaffle);
                    }
                    _ => self.entities.iter_mut().find(|e| e.id == entity_id).unwrap().update(x, y, vx, vy, has_snaffle)
                }
            }
            let entities_to_remove = self.entities.iter()
                                         .filter(|e| {
                                             e.entity_type == EntityType::Snaffle &&
                                                 existing_snaffles.iter().all(|s| s.id != e.id)
                                         }).map(|e| e.id).collect::<Vec<i32>>();
            let new_entities = self.entities.iter()
                                   .filter(|e1| {
                                       entities_to_remove.iter().all(|&id| e1.id != id)
                                   }).cloned().collect::<Vec<Entity>>();
            self.entities = new_entities;
        }
        self.set_targets();
    }
    pub fn act_turn(&mut self) {
        let mut magic_left = self.magic;
        for wizard in &self.wizards() {
            match self.optimal_action(&wizard, &magic_left) {
                ActionType::Throw => {
                    let dest: Vector2 = self.throw_destination(wizard);
                    self.throw_action(
                        &dest,
                        self.throw_power(wizard, &dest),
                    );
                }
                ActionType::Magic => {
                    let target: Entity = self.magic_target();
                    let dest: Vector2 = self.magic_destination(&target);
                    let magic_power = self.magic_power(&target, &dest, magic_left);
                    self.magic_action(
                        target.id,
                        &dest,
                        magic_power,
                    );
                    magic_left -= magic_power;
                }
                ActionType::Move => {
                    let dest: Vector2 = self.move_destination(wizard);
                    self.move_action(
                        &dest,
                        self.thrust_power(wizard, &dest),
                    )
                }
            }
        }
    }
    fn optimal_action(&self, wizard: &Entity, magic_left: &i32) -> ActionType {
        if wizard.has_snaffle {
            ActionType::Throw
        } else if self.should_magic(magic_left) {
            ActionType::Magic
        } else {
            ActionType::Move
        }
    }
    fn should_magic(&self, magic_left: &i32) -> bool {
        let close_to_limit = 1500.0;
        // Close to target or own goal
        if *magic_left > 15 &&
            (self.snaffles().iter().any(|s|
                self.target_goal.destination_is_close(s, close_to_limit)) ||
                self.snaffles().iter().any(|s|
                    self.own_goal.destination_is_close(s, close_to_limit))) {
            true
        } else if *magic_left > MAX_MAGIC / 2 {
            true
        } else {
            false
        }
    }
    fn throw_destination(&self, wizard: &Entity) -> Vector2 {
        let other_wizard_dest = self.other_wizard(wizard).collider.destination();
        //Bludgers hit other wizard next turn
        if self.bludgers().iter().any(|b| b.future().collider.collides(
            &self.other_wizard(wizard).future().collider
        )) {
            self.target_goal.random_inside_goal()
            //other wizard is close && its destination distance from goal is closer
            //than self => pass to other wizard
            //ToDo check that nothing is in between
        } else if other_wizard_dest.distance(wizard.collider.pos) < 1500. &&
            other_wizard_dest.distance(self.target_goal.center()) <
                wizard.collider.pos.distance(self.target_goal.center()) {
            other_wizard_dest
        } else {
            self.target_goal.random_inside_goal()
        }
    }
    fn throw_power(&self, wizard: &Entity, dest: &Vector2) -> i32 {
        let power_needed = wizard.collider.pos.distance(dest.clone()) * 0.75 / 0.5;
        if power_needed as i32 >= MAX_POWER {
            MAX_POWER
        } else {
            power_needed as i32
        }
    }
    fn magic_target(&self) -> Entity {
        // Since should magic is about "close to target or own goal", let's find closest to either
        let mut snaffles = self.snaffles();
        // Return random opponent if no snaffles
        if snaffles.len() == 0 {
            return self.opponents().first().cloned().unwrap();
        }
        snaffles.sort_by(|a, b| {
            (a.collider.destination().distance(self.target_goal.center()) as i32).cmp(
                &(b.collider.destination().distance(self.target_goal.center()) as i32)
            )
        });
        let closest_to_target = snaffles.first().cloned().unwrap();
        snaffles.sort_by(|a, b| {
            (a.collider.destination().distance(self.own_goal.center()) as i32).cmp(
                &(b.collider.destination().distance(self.own_goal.center()) as i32)
            )
        });
        let closest_to_own_goal = snaffles.first().cloned().unwrap();
        if closest_to_target.collider.pos.distance(self.target_goal.center()) <
            closest_to_own_goal.collider.pos.distance(self.own_goal.center()) {
            closest_to_target
        } else {
            closest_to_own_goal
        }
    }
    fn magic_destination(&self, target: &Entity) -> Vector2 {
        let wizards = self.wizards();
        //Take their future positions
        let wiz1 = wizards[0].clone().future();
        let wiz2 = wizards[1].clone().future();
        let wiz1_is_ahead = wiz1.collider.pos.distance(self.target_goal.center()) <
            target.collider.pos.distance(self.target_goal.center());
        let wiz2_is_ahead = wiz2.collider.pos.distance(self.target_goal.center()) <
            target.collider.pos.distance(self.target_goal.center());
        let wiz1_dist = wiz1.collider.pos.distance(target.collider.pos);
        let wiz2_dist = wiz2.collider.pos.distance(target.collider.pos);
        //Bludgers hit any wizard next turn
        if self.bludgers().iter().any(|b| {
            b.future().collider.collides(&wiz1.collider) ||
                b.future().collider.collides(&wiz2.collider)
        }) {
            self.target_goal.random_inside_goal()
            //Target is close to goal, shoot at goal
        } else if self.target_goal.destination_is_close(target, 2000.) {
            self.target_goal.random_inside_goal()
            //Target is closer to wiz1 than wiz1 && wiz1 is closer to goal => pass to wiz1
        } else if wiz1_dist < wiz2_dist && wiz1_is_ahead {
            wiz1.collider.pos
            //Second wizard is closer to target && closer to goal than target
        } else if wiz2_dist < wiz1_dist && wiz2_is_ahead {
            wiz2.collider.pos
        } else {
            self.target_goal.random_inside_goal()
        }
    }
    fn magic_power(&self, target: &Entity, dest: &Vector2, magic_left: i32) -> i32 {
        magic_left
    }
    fn move_destination(&mut self, wizard: &Entity) -> Vector2 {
        if wizard.target.is_some() {
            let target_id = wizard.target.unwrap();
            let target = self.entities.iter().find(|e| e.id == target_id)
                             .cloned().unwrap();
            let destination = target.collider.destination();
            destination
        } else {
            Vector2::new(WIDTH as f32 / 2., HEIGHT as f32 / 2.)
        }
    }
    fn thrust_power(&self, wizard: &Entity, dest: &Vector2) -> i32 {
        let thrust_needed = wizard.collider.pos.distance(dest.clone()) *
            wizard.collider.friction / wizard.collider.mass;
        if thrust_needed as i32 >= MAX_THRUST {
            MAX_THRUST
        } else {
            thrust_needed as i32
        }
    }

    fn set_targets(&mut self) {
        let snaffles = self.snaffles();
        let clone = self.clone();
        //Mutable reference to entities (Wizards)
        let mut wizards: Vec<&mut Entity> = self.entities.iter_mut()
                                                .filter(|e| e.entity_type == EntityType::Wizard)
                                                .collect();
        //Reset targets
        wizards[0].set_target(None);
        wizards[1].set_target(None);
        let pos1 = wizards[0].collider.pos;
        let pos2 = wizards[1].collider.pos;
        let closest_to_w1 = clone.closest_snaffle(pos1);
        let closest_to_w2 = clone.closest_snaffle(pos2);
        if snaffles.len() >= 1 {
            let closest1 = closest_to_w1.unwrap();
            let closest2 = closest_to_w2.unwrap();
            if snaffles.len() == 1 {
                //Same target
                wizards[0].set_target(Some(closest1.id));
                wizards[1].set_target(Some(closest1.id));
            } else if snaffles.len() > 1 {
                if closest1.id == closest2.id {
                    //Since closest to both is the same, choose wizard that's closer
                    if closest1.collider.pos.distance(wizards[0].collider.pos) <
                        closest1.collider.pos.distance(wizards[1].collider.pos) {
                        wizards[0].set_target(Some(closest1.id));
                    } else {
                        wizards[1].set_target(Some(closest1.id));
                    }
                } else {
                    wizards[0].set_target(Some(closest1.id));
                    wizards[1].set_target(Some(closest2.id));
                }
            }
        }
    }
    fn other_wizard(&self, wizard: &Entity) -> Entity {
        self.wizards().iter().find(|e| e.id != wizard.id).cloned().unwrap()
    }
    fn move_action(&self, dest: &Vector2, thrust: i32) {
        println!("{} {} {} {} MOVING", "MOVE", dest.x as i32, dest.y as i32, thrust)
    }
    fn throw_action(&self, dest: &Vector2, power: i32) {
        println!("{} {} {} {} THROWING", "THROW", dest.x as i32, dest.y as i32, power)
    }
    fn magic_action(&mut self, target_id: i32, dest: &Vector2, magic_power: i32) {
        println!("{} {} {} {} {} DOING SPELLS LOL", "WINGARDIUM", target_id, dest.x as i32, dest.y as i32, magic_power)
    }
    fn entities_of_type(&self, entity_type: EntityType) -> Vec<Entity> {
        self.entities.iter()
            .filter(|e| e.entity_type == entity_type).cloned().collect()
    }
    fn wizards(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Wizard) }
    fn opponents(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Opponent) }
    fn bludgers(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Bludger) }
    fn snaffles(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Snaffle) }
    fn closest_snaffle(&self, pos: Vector2) -> Option<Entity> {
        let snaffles = self.snaffles();
        if snaffles.len() == 0 { None } else {
            snaffles.iter().min_by(|a, b| {
                (a.collider.pos.distance(pos) as i32).cmp(
                    &(b.collider.pos.distance(pos) as i32)
                )
            }).cloned()
        }
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