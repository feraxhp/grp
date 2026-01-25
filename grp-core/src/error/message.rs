use crate::error::types::ErrorType;

impl ErrorType {
    pub(crate) fn get_message(&self) -> String {
        match self {
            ErrorType::Path404 => "File or Directory Not Found",
            ErrorType::Obj404 => "Object not found",
            ErrorType::UsettingsParsing => "Invalid config structure",
            ErrorType::ResponseParsing => "Failed at parsing response",
            ErrorType::PaginationErrors => "Pagination process failed",
            ErrorType::Unknown => "Unknown error occurred",
            ErrorType::Unauthorized => "Unauthorized access",
            ErrorType::Incomplete => "Missing required arguments",
            ErrorType::BadTokenScope => "Invalid token permissions",
            ErrorType::NotOwnerFound => "User not found",
            ErrorType::NotOrganizationFound => "Organization not found",
            ErrorType::LocalRepoNotFound |
            ErrorType::NotRepoFound => "Repository not found",
            ErrorType::AlreadyExists => "Already exists",
            ErrorType::FetchFailed => "Failed during fetch",
            ErrorType::Unsupported => "Unsuported action",
        }.to_string()
    }
}