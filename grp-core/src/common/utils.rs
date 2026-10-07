use futures::future::Ready;

use crate::Error;
use crate::structs::PaginatorResult;


pub fn skip_empty<T>(res: &Result<PaginatorResult<Vec<T>>, Error>) -> Ready<bool> {
    let keep = match res {
        Ok(batch) => !batch.result.is_empty(),
        Err(_) => true,
    };
    
    futures::future::ready(keep)
}