# gol (Game Of Life)

[![Static Badge](https://img.shields.io/badge/See_on-handmade.network-blue)](https://handmade.network/p/883/game-of-life)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/doomed-neko/gol/rust.yml)

This is my implementation of [Conway's Game Of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life), built in rust and uses [raylib](https://raylib.com) for the display.

### features

- Hold down the `space` key to see the generations progress or use the `N` key
  to step 1 generation at a time
- Click on any cell to toggle it's state
- Use the +/- keys to change the target FPS by 1, or hold down [/] keys for
  continuous change
- Use the 1..0 keys to set the target FPS to 10, 20, 30,etc..
- Use the `U` key to set the target FPS to _**unlimited**_
- Use the `R` key to the current canvas
- Use the `C` key to clear the canvas

![1787255921697496397.png](https://assets.media.handmade.network/f9fd2960-98f4-4a57-a238-b112028ae512/1787255921697496397.png)
