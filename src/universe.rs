pub trait Universe {
    fn set_bit(&mut self, x: isize, y: isize);

    fn get_bit(&self, x: isize, y: isize) -> u16;

    fn run_step(&mut self);
}
