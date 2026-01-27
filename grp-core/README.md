# grp-core

This library is meant to help interacting with a unique interface for the API of difrent 
git platforms, it abstracts the internal use of every (supported) platform api and creates 
and interface to interact with them.

## suported platforms
- GitHub 
- Gitea 
- Codeberg 
- Forgejo 
- GitLab

## Suported actions
### Repositories
- List 
- Create 
- Delete 
### Organizations
- List 
- Create (github and gitea could reject the creation, depending on the internal api politics) 
- Delete
### Authentication
- token authentication
### Aditional
You can also generate custom requests for every platform, and the authentication will be automaticly mannaged for you.

## examples
**See**: [feraxhp/grp](https://github.com/feraxhp/grp) to a full feature example of these implementations.