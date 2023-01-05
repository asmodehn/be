# Bevy Editor

A code Editor based on Bevy.

The idea is that having an ECS as the fundation for a code editor will enable adding advanced i/o behavior,
while maintaining efficient computation for fast user feedback.

The aim is to bring coding / software development and video game experience closer together...

Roadmap:
- [ ] Basic Text Editing
- [ ] Basic Terminal interaction / integration
- [ ] Basic Repl interaction / integration
- [ ] Basic Test interaction / integration
- [ ] Some Tutorial about writing your own plugin (system + component for adding feature)
- [ ] Colored text support
- [ ] Double Pane (horizontal or vertical)
- [ ] Syntax highlighting
- [ ] Syntax limiting (based on syntax rules, allow key input or not...)
- [ ] Your Programming Language Experiments...

## Development

First setup your environment for application development with the proper version of rust needed.
[Bevy has a nice documentation](https://bevyengine.org/learn/book/getting-started/setup/), read it.

We advise you setup [direnv](https://direnv.net/) with [asdf](https://asdf-vm.com/) on your machine,
to help setting everything correctly to work on this application.

Read the documentation of these tools and use them. It is well worth the effort.

After cloning the repo, and making sure you run `direnv allow` on the worktree, you can **let it be**:

```
$ cargo run
```


