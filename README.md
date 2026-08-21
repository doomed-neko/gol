# gol (Game Of Life)

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/doomed-neko/gol/rust.yml)

This is my implementation of [Conway's Game Of Life]
(en.wikipedia.org/wiki/Conway's_Game_of_Life), built in rust and uses
[raylib](raylib.com) for the display.

## keybinds and features

- Hold down the `space` key to see the generations progress or use the `N` key
  to step 1 generation at a time
- Left click on any cell to make it alive
- Right click on any cell to kill it
- Use the mouse wheel to zoom in/out
- Use the mouse wheel button to pan
- Use the `+`/`-` keys to change the target FPS by 1, or hold down [/] keys for
  continuous change
- Use the `,`/`.` keys to change the cell size by 1
- Use the `1`..`0` keys to set the target FPS to 10, 20, 30,etc..
- Use the `U` key to set the target FPS to _**unlimited**_
- Use the `R` key to the current canvas
- Use the `C` key to clear the canvas
- Use the `I` key to toggle status text in the gui
- Use the `Q` key to quit

## Command line options

```
  -?   --help             Print helo
  -n   --nogui            Run N generations without gui and exit
  -i   --hide-stats       Hide the stats text
  -f   --fill-chance      Set the random fill chance [0,1]
  -w   --window-width     Set the window width
  -h   --window-height    Set the window width
  -s   --tile-size        Set the tile size
  -v   --vsync            Sync fps to screen refresh rate
  -r   --fps              Set the rendering target fps
```

![1787255921697496397.png](https://assets.media.handmade.network/f9fd2960-98f4-4a57-a238-b112028ae512/1787255921697496397.png)
