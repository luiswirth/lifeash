pub trait Universe {
    fn set_bit(&mut self, x: isize, y: isize);

    fn run_step(&mut self);
}
