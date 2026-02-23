
## Configuration

The grp configurations are store in json. the first time you run
grp, it will create the configurations file with the _basic config_.

> if you prefer a guided configuration from the cli use: `grp config add`

the location of the config folder follows the system rules
- linux: `$XDG_CONFIG_HOME/girep/config.json` or `$HOME/.config/girep/config.json`
- windows `%APPDATA%/girep/config.json`
- mac: `$HOME/Library/Application Support/girep`

the basic config structure looks like this:
```json
{
  "default": "",
  "pconf": []
}
```

### Explanation
grp manage the platforms in objets called pcofs. in every pconf you have to add

- **name**: Is the name for the pconf, it is used to determine the platform. 
- **owner**: Is the username that will use by default to request in the platform.
- **token**: Is a user generated token used to authenticate the request.
- **type**: type of the platform. currently allows `github`, `gitea` and `gitlab`.
- **endpoint**: the endpoint to make the request 
  - examples:
    - `api.github.com`: for GitHub.
    - `gitea.com`: for Gitea.
    - `tea.example.com`: for Gitea on custom host.
    - `localhost:3244`: for gitea on localhost. (it might not work, as it only supports https)

### Example

```json
{
  "default": "gh",
  "pconf": [
      {
      "name": "gh",
      "owner": "feraxhp",
      "token": "<token generated>",
      "type": "github",
      "endpoint": "api.github.com"
    },
    {
      "name": "tea",
      "owner": "feraxhp",
      "token": "<token generated>",
      "type": "gitea",
      "endpoint": "tea.example.com"
    },
    {
      "name": "glab",
      "owner": "feraxhp",
      "token": "<token generated>",
      "type": "gitlab",
      "endpoint": "gitlab.example.com"
    }
  ]
}
```
