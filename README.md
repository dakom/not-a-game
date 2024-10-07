# [Play Now (desktop only)](https://dakom.github.io/not-a-game)
# [Read the article](https://dakom.github.io/not-a-game/media/html/about.html)

# Not a game

Some mixture of therapy, art, activism, and game development, related to the war in Israel - specifically commemorating October 7th (this was released exactly 1 year later).

Best to read the article or just dive in to get the gist. It's some form of self expression, by making a browser game in Rust with an explanatory essay, in a way that feels cathartic for me.

I began the idea many months ago, but didn't feel like anyone would get it or read it... since this was mainly for my own way of coming to grips with the world I see, kinda like writing some thoughts on a napkin, it went unpublished. I'm now polishing it up and putting it out there, maybe someone will appreciate it.

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