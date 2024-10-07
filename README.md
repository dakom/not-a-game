# [Play Now](https://dakom.github.io/not-a-game)  
# [Read the article](https://dakom.github.io/not-a-game/media/html/about.html)

# Not a game

Best to read the article or just dive in to get the gist. It's some form of self expression and statements about the war in Israel, by making a browser game in Rust, commemorating October 7th one year later in a way that feels cathartic for me.

I began the idea many months ago, but didn't feel like anyone would get it or read the article... since it was mainly for me, it went unpublished. I'm now polishing it up and putting it out there, maybe someone will appreciate it.

## Tech

* [Shipyard ECS](https://github.com/leudz/shipyard)
* [Raw WebGL Renderer](./src/renderer)
* [Pixel-Perfect collision detection (in GPU)](./src/collision/intersection.rs#L74)
* [Dominator UI](https://github.com/Pauan/rust-dominator)
* 100% Pure Rust :D

## Development

* Install all the stuff (Trunk, Rust, Taskfile)
* `task dev`

## TODO

* Bring down the draw calls (can bring it _way_ down)
* Add music?
* Transition screens
* Ramp up the number of missiles over time