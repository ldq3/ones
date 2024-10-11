pub trait Segment {
    fn get_base(&self) -> usize;
    fn get_limit(&self) -> usize;
    fn set_base(&mut self, base: usize);
    fn set_limit(&mut self, limit: usize);
}

pub trait SegmentTable {
    fn get_base(&self) -> usize;
    fn get_limit(&self) -> usize;
    fn set_base(&mut self, base: usize);
    fn set_limit(&mut self, limit: usize);
}

// Growth Direction
