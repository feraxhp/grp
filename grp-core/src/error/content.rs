use crate::error::{specific as sp, types::ErrorType};

impl ErrorType {
    pub(crate) fn map_content(&self, vec: Vec<String>) -> Vec<String> {
        match self {
            ErrorType::Obj404 => sp::obj404::content(vec),
            ErrorType::Path404 => sp::path404::content(vec),
            ErrorType::UsettingsParsing => sp::usettings_parsing::content(vec),
            ErrorType::ResponseParsing => sp::response_parsing::content(vec),
            ErrorType::PaginationErrors => sp::pagination_errors::content(vec),
            ErrorType::Unknown => sp::unknown::content(vec),
            ErrorType::Unauthorized => sp::unauthorized::content(vec),
            ErrorType::Incomplete => sp::incomplete::content(vec),
            ErrorType::BadTokenScope => sp::bad_token_scope::content(vec),
            ErrorType::NotOwnerFound => sp::not_owner_found::content(vec),
            ErrorType::NotOrganizationFound => sp::not_organization_found::content(vec),
            ErrorType::LocalRepoNotFound => sp::local_repo_not_found::content(vec),
            ErrorType::NotRepoFound => sp::not_repo_found::content(vec),
            ErrorType::AlreadyExists => sp::already_exist::content(vec),
            ErrorType::FetchFailed => sp::fetch_failed::content(vec),
            ErrorType::Unsupported => sp::unsupported::content(vec),
        }
    }
}