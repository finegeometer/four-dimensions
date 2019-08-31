A four-dimensional game. Run around and eat the tesseracts by walking into them!

# Status

The performance isn't great, but the game is playable. (I have successfully eaten all 81 tesseracts.)

## Todo

Fix the known-to-be-incorrect code. Also fix other bugs.

Make the game better. (Win screen, "nom" sound)

Do something with textures. I don't know what, but something.

Document code better.

Seperate concerns more:

	display / input (Different for wasm vs native? Should I add native? Make own module, so can swap out)
	3D -> 2D (depends on display system)

	Maybe a translation from inputs to controls?

	game logic

Integrate with VR.

More optimization is always good!

# Controls

Click above the fps counter to start.

Look left/right: Move mouse left/right

Look up/down: Move mouse up/down

Look ana/kata: Scroll up/down


Move forward/backward: W/S keys

Move left/right: A/D keys

Move up/down: Space/Shift keys

Move ana/kata: Q/E keys


Rotate 3D screen left/right: left/right arrow keys

Rotate 3D screen up/down: up/down arrow keys

# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
