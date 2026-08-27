# Auto Sort

Minecraft [Paper] server plugin for automatically sorting items in chests.

This plugin sorts items following the same order as the Creative mode
inventory. To ensure sorting is available to all players, this plugin uses a
named in-game item to trigger sort requests. Picking up the target item causes
all items in the chest to move to their sorted position.

## Quickstart

**Requires GNU Make 4.4.1, Gradle 9.1.0, OpenJDK 25.0.3, cargo 1.99.0, and
Python 3.14.5.**

Use `make` to generate all of the necessary files:

```bash
make generate
```

Copy the plugin JAR from `./build/libs/auto-sort.jar` to your server's
`plugins` directory:

```bash
cp "./build/libs/auto-sort.jar" "<PATH_TO_YOUR_SERVER>/plugins/AutoSort.jar"
```

Finally, restart the server to use the plugin. To sort a chest, you need to
have a `Paper` named `Sort` in your player's inventory. Picking up the paper
triggers a server-side sort request on the currently-open top inventory.

## Installation

Instructions for how to set up and use this plugin.

### Generate the Material List

**Requires Python 3.14.5.**

First, create and activate a new [Virtual Environment]:

```bash
# Create a new virtual environment.
python3 -m venv .venv
```

Install the required dependencies:

```bash
./.venv/bin/python3 -m pip install requests==2.3.0 lxml==5.3.2 bs4==0.0.2
```

This script outputs a newline-terminated list of `Material` constants from the
[SpigotMC] website. Redirect the contents of stdout to a file named
`materials.txt`:

```bash
mkdir ./data
./scripts/materials.py > ./data/materials.txt
```

### Generate `SortKey.java`

**Requires cargo 1.99.0.**

The following command takes the `materials.txt` file generated in the previous
step and passes it to `generate-code` to create `SortKey.java`:

```bash
cargo run --release -- generate-code \
    --input ./data/materials.txt \
    --profile ./profiles/Default.toml \
    > ./src/main/java/io/github/nicdgonzalez/autosort/SortKey.java
```

### Build the Plugin

**Requires Gradle 9.1.0 and OpenJDK 25.0.3.**

Use gradle to compile the plugin:

```bash
./gradlew build
```

### Install the Plugin

Copy the plugin JAR from `./build/libs/auto-sort.jar` to your server's
`plugins` directory:

```bash
cp "./build/libs/auto-sort.jar" "<PATH_TO_YOUR_SERVER>/plugins/AutoSort.jar"
```

Finally, restart the server to use the plugin. To sort a chest, you need to
have a `Paper` named `Sort` in your player's inventory. Picking up the paper
triggers a server-side sort request on the currently-open top inventory.

[paper]: https://papermc.io/
[spigotmc]: https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/Material.html
[virtual environment]: https://docs.python.org/3/library/venv.html
