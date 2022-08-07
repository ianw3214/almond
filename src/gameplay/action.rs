use crate::gameplay::stats::*;

// StatModifier ------------------------------------------
#[derive(Debug)]
pub struct StatModifier {
    pub stat : Stat,
    pub modifier : f32
}

// ActionEffect ------------------------------------------
#[derive(Debug)]
pub enum ActionEffect {
    Move,
    Damage(StatModifier)
}

// Action ------------------------------------------
#[derive(Debug, Default)]
pub struct Action {
    pub range : i32,
    pub effects : Vec<ActionEffect>
}