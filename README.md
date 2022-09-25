# Chestnut

Some sort of ~~strategy~~ city-building game I guess (I need to make up my mind)

To make SDL2 work on windows, [follow the instructions here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc)

To make SDL2_ttf work, follow the same instructions but for the [SDL2 ttf library instead](https://www.libsdl.org/projects/SDL_ttf/release/)

## Some sort of TODOs

- [x] Buildings
  - [ ] separate construction sprites
  - [ ] construction highlight
- [ ] map generation (tiles)
- [ ] resource system (game)
- [ ] UI system
  - [ ] load UI from json file (maybe? investigate)
  - [ ] scaling system for consistency
- [ ] Tech trees
- [ ] Resource manager (engine)
  - [ ] texture loading
  - [ ] font engine
- [x] Delta time management
- [ ] testing????
- [ ] collisions
  - [ ] seperate collision box vs 'selection' box
- [ ] camera system
- [ ] serialization
  - [ ] game state serialization
  - [ ] component serialization
- [x] rendering order
- [ ] text rendering