use crate::structs::PaginatorResult;


impl<T> PaginatorResult<T> {
    pub fn parse_result<H>(&self, result: H) -> PaginatorResult<H> {
        PaginatorResult { 
            result, 
            pager: self.pager.clone() 
        }
    }
}
