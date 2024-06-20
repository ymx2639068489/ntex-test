

pub mod company;

pub trait Pager {
    fn get_page(&self) -> i64;
    fn get_page_size(&self) -> i64;
    fn get_offset(&self) -> i64 {
        (self.get_page() - 1) * self.get_page_size()
    }
    fn get_limit(&self) -> i64 {
        self.get_page_size()
    }
}
