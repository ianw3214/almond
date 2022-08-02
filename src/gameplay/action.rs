// ActionEffect ------------------------------------------
#[derive(Debug)]
pub enum ActionEffect {
    Move,
    Damage(i32)
}

// Action ------------------------------------------
#[derive(Debug, Default)]
pub struct Action {
    pub range : i32,
    pub effects : Vec<ActionEffect>
}