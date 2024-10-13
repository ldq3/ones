pub mod frame;
pub mod table;

pub mod config {
    
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
