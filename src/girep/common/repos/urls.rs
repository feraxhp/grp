use crate::girep::common::users::structs::UserType;
use crate::girep::platform::Platform;


impl Platform {
    pub async fn url_list_repos<S: AsRef<str>>(&self, user_type: &UserType, endpoint: &S) -> String {
        // println!("{:#?}", &user_type);
        match (user_type, self) {
            (UserType::LoggedUser(_), Platform::Gitea) |
            (UserType::LoggedUser(_), Platform::Codeberg) |
            (UserType::LoggedUser(_), Platform::Github) => {
                format!("{}/user/repos", self.get_base_url(endpoint))
            }
            (UserType::LoggedOrg(user), Platform::Gitea) |
            (UserType::LoggedOrg(user), Platform::Codeberg) |
            (UserType::LoggedOrg(user), Platform::Github) => {
                format!("{}/orgs/{}/repos", self.get_base_url(endpoint), user.name)
            }
            (UserType::UnloggedUser(user), Platform::Gitea) |
            (UserType::UnloggedUser(user), Platform::Codeberg) |
            (UserType::UnloggedUser(user), Platform::Github) => {
                format!("{}/users/{}/repos", self.get_base_url(endpoint), user.name)
            }
            (UserType::LoggedUser(_), Platform::Gitlab) => {
                format!("{}/projects?membership=true", self.get_base_url(endpoint))
            }
            (UserType::LoggedOrg(u), Platform::Gitlab) |
            (UserType::UnloggedOrg(u), Platform::Gitlab)  => {
                format!("{}/groups/{}/projects", self.get_base_url(endpoint), u.id)
            }
            (UserType::UnloggedUser(u), Platform::Gitlab) => {
                format!("{}/users/{}/projects", self.get_base_url(endpoint), u.id)
            }
            _ => todo!("Not implemented")
        }
    }
    
    pub async fn url_create_repo<S: AsRef<str>>(&self, user_type: &UserType, endpoint: &S) -> String {
        // println!("{:#?}", &user_type);
        match (user_type, self) {
            (UserType::LoggedUser(_), Platform::Gitea) |
            (UserType::LoggedUser(_), Platform::Codeberg) |
            (UserType::LoggedUser(_), Platform::Github) => {
                format!("{}/user/repos", self.get_base_url(endpoint))
            }
            (UserType::LoggedOrg(user), Platform::Gitea) |
            (UserType::LoggedOrg(user), Platform::Codeberg) |
            (UserType::LoggedOrg(user), Platform::Github) => {
                format!("{}/orgs/{}/repos", self.get_base_url(endpoint), user.name)
            }
            (UserType::UnloggedUser(user), Platform::Gitea) |
            (UserType::UnloggedUser(user), Platform::Codeberg) |
            (UserType::UnloggedUser(user), Platform::Github) => {
                format!("{}/users/{}/repos", self.get_base_url(endpoint), user.name)
            }
            (utype, Platform::Gitlab) => {
                let u = utype.get_user();
                format!("{}/projects?namespace_id={}", self.get_base_url(endpoint), u.id)
            }
            _ => todo!("Not implemented")
        }
    }
    
    pub async fn url_delete_repo<S: AsRef<str>>(&self, owner: &S, repo: &S, endpoint: &S) -> String {
        match self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Gitea => format!("{}/repos/{}/{}", self.get_base_url(endpoint), owner.as_ref(), repo.as_ref()),
            Platform::Gitlab => {
                format!("{}/projects/{}", self.get_base_url(endpoint), owner.as_ref())
            },
        }
    }
    

}