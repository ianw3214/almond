# Chestnut

~~Some sort of city building game I guess~~

Some sort of strategy game I guess

To make SDL2 work on windows, [follow the instructions here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)

Some sort of TODO list:

- **GAMEPLAY**
  - [x] Grid based movement
    - [ ] Tweened movement system
  - [ ] Turn ordering
- **RENDERING**
  - [x] Handle separation between view space & world space
  - [x] Basic camera system to handle scale
    - [ ] Extend camera system for custom camera position
  - [ ] Sorted rendering based on y position/height
- **UI**
  - [ ] Custom mouse cursor rendering
  - [x] Render currently hovered grid
  - [ ] UI to show info for currently selected entity
- **CLEANUP**
  - [ ] Merge _info_ structs into something more unified