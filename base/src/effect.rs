pub trait Effect {
    fn apply(&self);
    fn description(&self) -> String;
}