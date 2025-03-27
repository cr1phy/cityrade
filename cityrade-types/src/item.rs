pub trait Item {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn price(&self) -> u32;
}
