# Contributions

## Git sintaxis

### Commit messages

- Use Markdown format to write commit messages.
- The title (# ) must be the first line of the commit message.
  - A 3 word max title that describes what did you change.
  - Must finish with a colon.
- The body must be separated in sub tittles.
- the changes must be added with the sintaxis:
    - `+` for new features or new code.
    - `-` for removed features or code.
    - `*` for modified features or code. 
    - `>` for renames or moves. 
- if the change is too tiny to have a title you can just add de change syntax.
  - considere open an issue to discuss the change. instead of a pull-request.

#### Example

```markdown
# Preparations for GitHub:

## Documentation:
    
  + CONTRIBUTING.md
  * README.md
  * dependencies.txt > dependencies.md
```

this is the commit message that is going to be added to the git log, to upload this changes.
