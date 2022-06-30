# Makai Kit

Collection of utilities for modifying Disgaea and other NIS games.

## Mod Loader Installation (Disgaea 6)

The cargo workspace must be built for x86_64-windows-msvc target.

1. Move `dinput8.dll` (`dll-injector-dinput8`) to the game dir
2. Move `makaikit-modloader-d6.dll` to `mkplugins` directory in game dir
3. Make a directory named `mods` in the game dir
4. Loose files go inside individual mod directories inside `mods`

If the game looks for `data/database/framework_cmd.dat`, and there exists a file
`mods/foobar/data/database/framework_cmd.dat`, the mod loader will intercept the
file load and load `foobar`'s file instead.
