# Chestnut

Some sort of strategy game I guess

To make SDL2 work on windows, [follow the instructions here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)

Some sort of TODO list:

- **STUFF**
  - [ ] Game design docs?? (kinda extra...)

- **GAMEPLAY**
  - [x] Grid based movement
    - [ ] Tweened movement system
  - [x] Basic entity attack
    - [x] Custom entity attacks
    - [ ] Attack properties
      - [ ] Range
      - [ ] Custom effects (damage, heal, etc..)
  - [ ] Entity stats
  - [x] Turn ordering
  - [ ] Enemy AI
- **RENDERING**
  - [x] Handle separation between view space & world space
  - [x] Basic camera system to handle scale
    - [ ] Extend camera system for custom camera position
  - [x] Render entities properly aligned to grid
    - [x] Sorted rendering based on y position/height
  - [ ] Proper animation fps
  - [x] Render entity health
  - [ ] Render background
  - [ ] Render attack info when selected
    - [ ] Range indicator
- **UI**
  - [ ] Custom mouse cursor rendering
  - [x] Render currently hovered grid
  - [ ] UI to show info for currently selected entity
    - [ ] Show entity name
    - [ ] Show entity stats
- **SYSTEMS**
  - [ ] JSON File loading
    - [ ] Character saves
    - [ ] Campaign saves (Combined with character saves?)
- **CLEANUP**
  - [ ] Merge _info_ structs into something more unified
  - [x] Proper error handling (good enough rn i guess)
  - [ ] Refactor systems to utilize [optional components](https://specs.amethyst.rs/docs/tutorials/08_join.html)
  - [ ] Full testing of current util functions
    - [ ] Investigate if other parts of the code is testable
  - [ ] Setup github actions for CI/CD