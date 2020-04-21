use std::io;

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

static WIDTH: i32 = 16001;
static HEIGHT: i32 = 7501;
static MAX_THRUST: i32 = 150;
static MAX_POWER: i32 = 500;
static MAX_MAGIC: i32 = 100;
static LOOKAHEAD: i32 = 3;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn add(&self, v2: Vector2) -> Vector2 {
        Vector2::new(self.x + v2.x, self.y + v2.y)
    }
    pub fn negate(&self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
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
    pub fn destination_turns(&self, turns: i32) -> Vector2 {
        let mut new_vel = self.vel.mul_num(self.friction);
        let mut new_pos = self.pos.add(new_vel);
        for _ in 1..turns {
            new_vel = self.vel.mul_num(self.friction);
            new_pos = new_pos.add(new_vel);
        }
        new_pos
    }

    pub fn velocity_turns(&self, turns: i32) -> Vector2 {
        let mut new_vel = self.vel.mul_num(self.friction);
        for _ in 1..turns {
            new_vel = self.vel.mul_num(self.friction);
        }
        new_vel
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
        self.collider.vel.y = vy as f32;
        self.has_snaffle = has_snaffle;
    }
    pub fn set_target(&mut self, target: Option<i32>) {
        self.target = target;
    }
    pub fn future_turns(&self, turns: i32) -> Entity {
        Entity {
            id: self.id,
            entity_type: self.entity_type.clone(),
            collider: Collider::new(
                self.collider.destination_turns(turns),
                self.collider.velocity_turns(turns),
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
                    Vector2::new(16000.0, 1750.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
                pole_bottom: Collider::new(
                    Vector2::new(16000.0, 5750.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
            }
        } else {
            Goal {
                pole_top: Collider::new(
                    Vector2::new(0.0, 1750.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
                pole_bottom: Collider::new(
                    Vector2::new(0.0, 5750.0),
                    Vector2::new(0.0, 0.0), 0., 0.0, 300.0, ),
            }
        }
    }
    pub fn destination_is_close(&self, entity: &Entity, close_to_limit: f32) -> bool {
        self.points_inside_goal(10).iter().any(|&point| {
            let dist_from_point = entity.collider.destination_turns(LOOKAHEAD).distance(point);
            dist_from_point < close_to_limit
        })
    }
    pub fn points_inside_goal(&self, num: usize) -> Vec<Vector2> {
        let div = num as f32;
        let mut points = vec![];
        let dist = self.pole_top.pos.distance(self.pole_bottom.pos) - self.pole_top.radius * 2.0;
        for i in 0..(num - 1) {
            points.push(Vector2::new(
                self.pole_bottom.pos.x,
                self.pole_top.pos.y + self.pole_top.radius + 200.0 + i as f32 * (dist / div),
            ))
        }
        points
    }
    pub fn center(&self) -> Vector2 { Vector2::new(self.pole_bottom.pos.x, 3750.0) }
    pub fn behind_goal(&self) -> Vector2 {
        let center = self.center();
        Vector2::new(center.x + if center.x == 0. { -2000. } else { 2000. }, center.y)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum ActionType {
    Throw,
    Move,
    Magic,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum TargetStrategy {
    ClosestToWizard,
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
                    self.throw_action(&dest, MAX_POWER);
                }
                ActionType::Magic => {
                    let target: Entity = self.magic_target();
                    let dest: Vector2 = self.magic_destination(&target);
                    let magic_power = self.magic_power(&target, &dest, magic_left);
                    self.magic_action(&target, &dest, magic_power);
                    magic_left -= magic_power;
                }
                ActionType::Move => {
                    let dest: Vector2 = self.move_destination(wizard);
                    self.move_action(&dest, MAX_THRUST)
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
        let close_to_limit = 2500.0;
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
        let wizard_future = wizard.clone().future_turns(LOOKAHEAD);
        let other_wizard_dest = self.other_wizard(&wizard_future).collider.destination_turns(LOOKAHEAD);
        let result = if wizard_future.collider.pos.distance(self.target_goal.center()) < 4000. {
            self.target_goal.behind_goal()
        } else if other_wizard_dest.distance(wizard_future.collider.pos) < 1500. &&
            other_wizard_dest.distance(self.target_goal.center()) <
                wizard_future.collider.pos.distance(self.target_goal.center()) &&
            !self.is_obstacles_in_between(&wizard_future.collider.pos, &other_wizard_dest) {
            other_wizard_dest
        } else if wizard_future.collider.pos.distance(self.target_goal.center()) > WIDTH as f32 / 2. {
            match self.open_destination_ahead(&wizard_future, 8000) {
                Some(dest) => dest,
                None => self.target_goal.behind_goal()
            }
        } else {
            self.target_goal.behind_goal()
        };
        result.add(wizard_future.collider.vel.negate())
    }
    fn magic_target(&self) -> Entity {
        // Since should magic is about "close to target or own goal", let's find closest to either
        let mut snaffles = self.snaffles();
        // Return random opponent if no snaffles
        if snaffles.len() == 0 {
            return self.opponents().first().cloned().unwrap();
        }
        snaffles.sort_by(|a, b| {
            (a.collider.destination_turns(LOOKAHEAD).distance(self.target_goal.center()) as i32).cmp(
                &(b.collider.destination_turns(LOOKAHEAD).distance(self.target_goal.center()) as i32)
            )
        });
        let closest_to_target = self.closest_snaffle(self.target_goal.center()).unwrap();
        let closest_to_own_goal = self.closest_snaffle(self.own_goal.center()).unwrap();
        if closest_to_target.collider.destination_turns(LOOKAHEAD).distance(self.target_goal.center()) <
            closest_to_own_goal.collider.destination_turns(LOOKAHEAD).distance(self.own_goal.center()) {
            closest_to_target
        } else {
            closest_to_own_goal
        }
    }
    fn magic_destination(&self, target: &Entity) -> Vector2 {
        let target_future = target.future_turns(LOOKAHEAD);
        let wizards = self.wizards();
        //Take their future positions
        let wiz1 = wizards[0].clone().future_turns(LOOKAHEAD);
        let wiz2 = wizards[1].clone().future_turns(LOOKAHEAD);
        let wiz1_is_ahead = wiz1.collider.pos.distance(self.target_goal.center()) <
            target_future.collider.pos.distance(self.target_goal.center());
        let wiz2_is_ahead = wiz2.collider.pos.distance(self.target_goal.center()) <
            target_future.collider.pos.distance(self.target_goal.center());
        let wiz1_dist = wiz1.collider.pos.distance(target_future.collider.pos);
        let wiz2_dist = wiz2.collider.pos.distance(target_future.collider.pos);
        //Target is close to goal, shoot at goal
        let result =
            if target_future.collider.pos.distance(self.target_goal.center()) < WIDTH as f32 / 2. {
                self.target_goal.behind_goal()
            } else {
                match self.open_destination_ahead(&target_future, 4000) {
                    Some(dest) => dest,
                    None => {
                        if wiz1_is_ahead && wiz2_is_ahead {
                            if wiz1_dist < wiz2_dist { wiz1.collider.pos } else { wiz2.collider.pos }
                        } else if wiz1_is_ahead
                        { wiz1.collider.pos } else if wiz2_is_ahead
                        { wiz2.collider.pos } else {
                            self.target_goal.behind_goal()
                        }
                    }
                }
            };
        result.add(target_future.collider.vel.negate())
    }
    fn open_destination_ahead(&self, target: &Entity, far: i32) -> Option<Vector2> {
        let future_pos = target.collider.destination_turns(LOOKAHEAD);
        // From top to bottom
        let multiplier = if self.team_id == 0 {
            1
        } else { -1 } as f32;
        let vertical_points_ahead = self.in_between_points(
            &Vector2::new(future_pos.x + multiplier * far as f32, 0.0),
            &Vector2::new(future_pos.x + multiplier * far as f32, 16000.0),
            10,
        );
        let obstacles: Vec<Entity> = self.obstacles().iter()
                                         .map(|o| o.future_turns(LOOKAHEAD).clone()).collect();
        //Filter vertical points to only those that don't have obstacles between target & point
        let possible_destinations = vertical_points_ahead.iter().filter(|p| {
            //Filter vertical positions with direct line of sight to target
            !self.in_between_colliders(&future_pos, p, 10).iter().any(|c| {
                obstacles.iter().any(|o| o.collider.collides(c))
            })
        }).cloned().collect::<Vec<Vector2>>();
        possible_destinations.iter().min_by(|&a, &b| {
            (a.distance(future_pos) as i32)
                .cmp(&(b.distance(future_pos) as i32))
        }).cloned()
    }
    fn magic_power(&self, target: &Entity, dest: &Vector2, magic_left: i32) -> i32 {
        let magic_needed = target.collider.destination_turns(LOOKAHEAD)
                                 .distance(dest.clone()) *
            target.collider.friction / target.collider.mass;
        if magic_needed as i32 >= magic_left {
            magic_left
        } else {
            magic_needed as i32
        }
    }
    fn move_destination(&mut self, wizard: &Entity) -> Vector2 {
        if wizard.target.is_some() {
            let target_id = wizard.target.unwrap();
            let target = self.entities.iter().find(|e| e.id == target_id)
                             .cloned().unwrap();
            let destination = target.collider.destination_turns(LOOKAHEAD);
            destination.add(wizard.collider.vel.negate())
        } else {
            Vector2::new(WIDTH as f32 / 2., HEIGHT as f32 / 2.)
        }
    }
    fn set_targets(&mut self) {
        let snaffles = self.snaffles();
        let clone_state = self.clone();
        let target_strategy = self.target_strategy();
        //Mutable reference to entities (Wizards)
        let mut wizards: Vec<&mut Entity> = self.entities.iter_mut()
                                                .filter(|e| e.entity_type == EntityType::Wizard)
                                                .collect();
        //Reset targets
        wizards[0].set_target(None);
        wizards[1].set_target(None);
        if snaffles.len() == 0 { return; }
        match target_strategy {
            TargetStrategy::ClosestToWizard => {
                let closest1 = clone_state.closest_snaffle(wizards[0].collider.pos);
                let mut closest2 = clone_state.closest_snaffle(wizards[1].collider.pos);
                if snaffles.len() > 1 {
                    let e1_id = closest1.clone().unwrap().id;
                    let e2_id = closest2.clone().unwrap().id;
                    if e1_id == e2_id {
                        closest2 = clone_state.second_closest_snaffle(e1_id, wizards[1].collider.pos);
                    }
                }
                if snaffles.len() == 1 {
                    let e1 = closest1.unwrap();
                    //Same target
                    wizards[0].set_target(Some(e1.id));
                    wizards[1].set_target(Some(e1.id));
                } else if snaffles.len() > 1 {
                    let e1 = closest1.unwrap();
                    let e2 = closest2.unwrap();
                    wizards[0].set_target(Some(e1.id));
                    wizards[1].set_target(Some(e2.id));
                }
            }
        };
    }
    fn target_strategy(&self) -> TargetStrategy {
        TargetStrategy::ClosestToWizard
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
    fn magic_action(&mut self, target: &Entity, dest: &Vector2, magic_power: i32) {
        println!("{} {} {} {} {} DOING SPELLS LOL", "WINGARDIUM", target.id, dest.x as i32, dest.y as i32, magic_power)
    }
    fn entities_of_type(&self, entity_type: EntityType) -> Vec<Entity> {
        self.entities.iter()
            .filter(|e| e.entity_type == entity_type).cloned().collect()
    }
    fn wizards(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Wizard) }
    fn opponents(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Opponent) }
    fn snaffles(&self) -> Vec<Entity> { self.entities_of_type(EntityType::Snaffle) }
    fn obstacles(&self) -> Vec<Entity> {
        self.entities.iter()
            .filter(|e| e.entity_type != EntityType::Wizard)
            .cloned().collect()
    }
    fn closest_snaffle(&self, pos: Vector2) -> Option<Entity> {
        self.snaffles().iter().min_by(|a, b| {
            (a.collider.destination_turns(LOOKAHEAD).distance(pos) as i32).cmp(
                &(b.collider.destination_turns(LOOKAHEAD).distance(pos) as i32)
            )
        }).cloned()
    }
    fn second_closest_snaffle(&self, ignore_id: i32, pos: Vector2) -> Option<Entity> {
        self.snaffles().iter().filter(|s| s.id != ignore_id)
            .min_by(|a, b| {
                (a.collider.destination_turns(LOOKAHEAD).distance(pos) as i32)
                    .cmp(&(b.collider.destination_turns(LOOKAHEAD).distance(pos) as i32))
            }).cloned()
    }
    fn is_obstacles_in_between(&self, start: &Vector2, end: &Vector2) -> bool {
        let obstacles = self.obstacles();
        self.in_between_colliders(start, end, 10).iter().any(|c| {
            obstacles.iter().any(|o| o.collider.collides(c))
        })
    }
    fn in_between_points(&self, start: &Vector2, end: &Vector2, num: i32) -> Vec<Vector2> {
        let mut points_int_between = vec![];
        let div = num as f32;
        let dist = start.distance(end.clone());
        let position = start.clone();
        let direction = position.direction(end.clone());
        for i in 1..num {
            let new_pos = position.add(
                direction.mul_num(i as f32 * dist / div)
            );
            points_int_between.push(new_pos);
        }
        points_int_between
    }

    fn in_between_colliders(&self, start: &Vector2, end: &Vector2, num: i32) -> Vec<Collider> {
        self.in_between_points(start, end, num).iter().map(|p| {
            Collider::new(
                p.clone(),
                Vector2::new(0., 0.), 0.75, 0.5, 150.,
            )
        }).collect()
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