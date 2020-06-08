pub trait Universe {
    fn set_bit(&mut self, x: i32, y: i32, alive: bool);

    fn get_bit(&self, x: i32, y: i32) -> u16;

    fn run_step(&mut self);
}
