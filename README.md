# Makai Kit

Collection of utilities for modifying Disgaea and other NIS games.

## Mod Loader Installation

Either extract the contents of a distribution zip for the particular game into
the installation directory of that game, or place a compiled `dinput8.dll` in
the base game directory and `makaikit-modloader-{GAME}.dll` in a `mkplugins`
subdirectory.

```
- Disgaea 7 Vows of the Virtueless/
  - Disgaea7.exe
  - dinput8.dll
  - mkplugins/
    - makaitkit-modloader-d7.dll
  - mods/
```

## Mod installation

Place the mod's directory in the `mods` subdirectory, for example:

```
- Disgaea 7 Vows of the Virtueless/
  - Disgaea7.exe
  - dinput8.dll
  - mkplugins/
  - ...
  - mods/
    - downloaded-mod-v1/
      - ...

```

## Mod Creation

Mods are loose-file directories placed under the `mods` subdirectory. All
directory names prefixed by 1 or more underscores (_) are **reserved** for this
purpose (e.g. don't name it `_generated`).

Example layout:

```
- mods/
  - my-mod-v1/
    - databases/
      - item/
        - 10000_ITEM_MYMOD_CUSTOM_01.json
    - files/
      - data/
        - sound/
          - CUSTOM_SOUND.nlsd
    - README.md
```

Mods are loaded in lexicographic order.

### Flat file replacement

All files in the `files` subdirectory of a mod will be used to replace file
requests in the virtual filesystem used by the game. For example, if the game
requests `data/sound/CUSTOM_SOUND.nlsd`, the mod loader will attempt to find the
file and substitute it by mod load order.

### Database patching

Database entries for supported databases will be combined from all mods into a
new generated database. These are placed in _generated/ on startup.

Database entries may come in 3 forms:

- Full JSON record. These are the output of the database unpacker. The file
extension for these must be .json.
- [RFC7368 JSON Merge-patch](https://datatracker.ietf.org/doc/html/rfc7386).
These patches will be applied in mod load order to accumulate the final database
record. The file extension of these must be .merge.json, and include either the
record ID as a prefix, or the record's Lua enum name (ideally both).
- [RFC6902 JSON Patch](https://jsonpatch.com/). These follow the same rules as
merge patches but must have the extension .patch.json.

### Script replacement

Only applies to Disgaea 6. Lua scripts in the `scripts` subdirectory of a mod
will be patched into a new `script.dat` on startup in mod load order.

## License

makaikit is made available under the terms of the GNU General Public License
3.0. Mods created for use with makaikit are not required to be made available
under the same license unless code from makaikit itself is used.
