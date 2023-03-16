# schemsearch
### A *simple* CLI tool to search in Sponge V2 Schematic files

---

## WARNING: This is a work in progress and is really simple right now. It will be improved in the future.
| Feature                | Status |
|------------------------|--------|
| Block search           | ✅      |
| Block data less search | ✅      |
| Tile entities search   | ❌      |
| Entities search        | ❌      |

---

## Reasoning
As a Developer of the [SteamWar.de Minecraft server](https://steamwar.de) we needed a way to search for schematics that contain a specific pattern.
Focused on "competitive Redstone building" stealing of other players techniques is a big problem, so this is a little side project of mine.

---

## Usage
The tool takes a pattern as it's first parameter, this should be a valid Sponge V2 Schematic File.
The second parameter is a list of schematics or directories to search in.
```bash
schemsearch-cli <pattern> <schematic...>
```

### Examples
Simple search
```bash
schemsearch-cli tests/endstone.schem tests/simple.schem
```

### Help
The rest of the valid parameters can be found by using the help command.
```bash
schemsearch-cli --help
```

---

## Building
This project is build using Rust for the CLI and library. It can be built using Cargo.
```bash
cargo build --release
```

### Features: schemsearch-sql
This feature is not enabled by default as it is a specific implementation for the SteamWar.de Minecraft server.
**You shouldn't enable this feature unless you know what you are doing.**

---

## License
This project is licensed under the AGPL-3.0 License. See the [LICENSE](LICENSE.txt) file for details.
