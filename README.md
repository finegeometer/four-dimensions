A four-dimensional game.

# Status

This currently only has a renderer.

It does not do exactly what I want; see problem below.

If I solve that problem, I will probably turn this into an actual game.

If I don't, I will probably abandon this project.


# Controls

You must click the screen to lock the pointer before mouse controls will work.

Look left/right: Move mouse left/right

Look up/down: Move mouse up/down

Look ana/kata: Scroll up/down


Move forward/backward: W/S keys

Move left/right: A/D keys

Move up/down: Space/Shift keys

Move ana/kata: Q/E keys


Rotate 3D screen left/right: left/right arrow keys

Rotate 3D screen up/down: up/down arrow keys

# Problem: 1D vs 2D textures

If you compile and run the code as it currently stands, it will render the scene with 1D textures (line-art).
I want it to instead use 2D textures (plane-art), but I have repeatedly been unsuccessful at implementing occlusion.
If I get 2D textures working properly, I will remove the 1D texture code.

To switch the code to 2D textures, several changes need to be made to the code.
All of these are marked by a comment containing the string "TXPRBLM".
- In `fn project` in the second `impl Mesh` in mesh.rs,
	- Turn off occlusion by removing the relevant line.
	- Replace the line `texcoord: ...` as marked in the code.
- In `fn cube` in `impl Facet` in mesh.rs, replace the contents of `texture: ...` as marked in the code.
- In `impl Space for Texture` in mesh/space.rs, change Dim from `na::U1` to `na::U2`
- Replace all of mesh/texture.rs with the commented code in that file.
- In render/to_tex.rs, replace the line `gl.draw_arrays(GL::LINES, ...);` with `gl.draw_arrays(GL::TRIANGLES, ...);`.

# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
