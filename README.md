# Chestnut

Some sort of strategy game I guess

To make SDL2 work on windows, [follow the instructions here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)

Some sort of TODO list:

- **GAMEPLAY**
  - [x] Grid based movement
    - [ ] Tweened movement system
  - [ ] Entity actions
  - [ ] Turn ordering
- **RENDERING**
  - [x] Handle separation between view space & world space
  - [x] Basic camera system to handle scale
    - [ ] Extend camera system for custom camera position
  - [x] Render entities properly aligned to grid
    - [x] Sorted rendering based on y position/height
  - [ ] Proper animation fps
- **UI**
  - [ ] Custom mouse cursor rendering
  - [x] Render currently hovered grid
  - [ ] UI to show info for currently selected entity
- **CLEANUP**
  - [ ] Merge _info_ structs into something more unified
  - [x] Proper error handling (good enough rn i guess)
  - [ ] Refactor systems to utilize [optional components](https://specs.amethyst.rs/docs/tutorials/08_join.html)
  - [ ] Full testing of current util functions
    - [ ] Investigate if other parts of the code is testable
  - [ ] Setup github actions for CI/CD