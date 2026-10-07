pub mod repo;
pub mod user;


use crate::Error;
use crate::Platform;
use crate::structs::Issue;
use crate::structs::PaginatorResult;


impl Platform {
    pub(self) fn get_issues(&self, response: Result<PaginatorResult<String>, Error>) -> Result<PaginatorResult<Vec<Issue>>, Error> {
        let pagginator = response?;
        let issues = Issue::from_text_array(&pagginator.result, &self)?;
        Ok(pagginator.parse_result(issues))
    }
}