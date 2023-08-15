# thud

*thud*, short for **thu**mbnail **d**irectory, is a tool that lets you generate directory thumbnails from images inside them. Useful for a Dolphin/KDE-like experience with GTK-based file browsers that don't show the contents of directories by default.

thud can be customized with *rules* and *strategies* that reside in [`~/.config/thud/config.toml`](./examples/config.toml). By default, it creates thumbnails from `cover.{png,jpg}` files.

![Screenshot of thud changing folder icons to cover images in nautilus.](./cover.png)

Tested and works in: [Caja](https://github.com/mate-desktop/caja), [Nemo](https://github.com/linuxmint/nemo), [Thunar](https://github.com/xfce-mirror/thunar), [Nautilus](https://github.com/GNOME/nautilus).

## Installation

### [NixOS](https://nixos.wiki/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

Add [`thud`](https://search.nixos.org/packages?channel=unstable&query=thud) to your `systemPackages` and rebuild.

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    thud
  ];
}
```

### [Arch Linux](https://archlinux.org/)

```fish
git clone https://github.com/donovanglover/thud && cd thud && makepkg -si
```

### Other distributions

```fish
git clone https://github.com/donovanglover/thud && cd thud
cargo build --release

sudo install -Dm755 ./target/release/thud /usr/bin/thud
sudo install -Dm644 ./assets/thud.thumbnailer /usr/share/thumbnailers/thud.thumbnailer
```

## Usage

```man
Usage: thud [OPTIONS] --input-directory <INPUT_DIRECTORY> --output-file <OUTPUT_FILE>

Options:
  -s, --size <SIZE>                        Size of the thumbnail to output [default: 128]
  -i, --input-directory <INPUT_DIRECTORY>  Directory to base the thumbnail off of
  -o, --output-file <OUTPUT_FILE>          Where to save the output image
  -v, --verbose                            Print what thud is doing
  -h, --help                               Print help (see more with '--help')
  -V, --version                            Print version
```

By default, directories with `cover.jpg` or `cover.png` will automatically generate cover images. You can customize this with `~/.config/thud/config.toml`.

## Contributing

Anyone can add their own strategies to thud by forking the repository and making a pull request.
