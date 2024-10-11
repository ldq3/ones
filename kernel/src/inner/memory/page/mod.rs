pub mod table;

pub trait FrameNum: Into<usize> + From<usize> {
    type PhysicalAddress: PhysicalAddress;

    fn physical_address(&self) -> Self::PhysicalAddress;
}

pub trait PhysicalAddress: From<usize> + Into<usize> {
    type F: FrameNum;

    fn frame_num(&self) -> Self::F;

    fn offset(&self) -> usize;

    #[inline]
    fn ceil_frame_num(&self) -> Self::F {
        let frame_num_int: usize = self.frame_num().into();
        
        if self.offset() == 0 {
            frame_num_int.into()
        } else {
            (frame_num_int + 1).into()
        }
    }
}

use core::mem::needs_drop;
// #FIXME: !Copy
pub trait Frame: Sized {
    fn new() -> Result<Self, ()>;
}

pub trait FrameManager {
    fn new(start: usize, end: usize) -> Self;
    fn init(&mut self);
    fn alloc() -> usize;
    fn dealloc(frame: usize);
}

pub trait PageNum: Into<usize> + From<usize> {}

pub trait VirtualAddress: From<usize> + Into<usize> {
    type P: PageNum;

    fn page_num(&self) -> Self::P;

    fn offset(&self) -> usize;

    #[inline]
    fn ceil_page_num(&self) -> Self::P {
        let page_num_int = self.page_num().into();

        if self.offset() == 0 {
            page_num_int.into()
        } else {
            (page_num_int + 1).into()
        }
    }
}
