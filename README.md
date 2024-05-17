# thud

*thud*, short for **thu**mbnail **d**irectory, is a tool that lets you generate directory thumbnails from images inside them. Useful for a [Dolphin](https://apps.kde.org/dolphin/)/[KDE](https://kde.org/)-like experience with [GTK](https://www.gtk.org/)-based [file](https://apps.gnome.org/Nautilus/) [browsers](https://docs.xfce.org/xfce/thunar/start) that don't show the contents of directories by default.

thud can be customized with *rules* and *strategies* that reside in [`~/.config/thud/config.toml`](./examples/config.toml). By default, it creates thumbnails from `cover.{png,jpg}` files.

![Screenshot of thud changing folder icons to cover images in nautilus.](./cover.png)

Tested and works in: [Caja](https://github.com/mate-desktop/caja), [Nemo](https://github.com/linuxmint/nemo), [Thunar](https://github.com/xfce-mirror/thunar), [Nautilus](https://github.com/GNOME/nautilus).

## Installation

### [NixOS](https://wiki.nixos.org/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

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

I don't use Arch Linux anymore, but I wrote a PKGBUILD for the `pacman` enjoyers out there. Feel free to add it to the AUR.

```fish
git clone https://github.com/donovanglover/thud -b 0.3.1 && cd thud && makepkg -si
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
Usage: thud [OPTIONS] <INPUT_DIRECTORY> <OUTPUT_FILE>

Arguments:
  <INPUT_DIRECTORY>  Directory to thumbnail
  <OUTPUT_FILE>      Where to save the output image

Options:
  -s, --size <INTEGER>  Thumbnail size [default: 128]
  -v, --verbose         Print what thud is doing
  -h, --help            Print help (see more with '--help')
  -V, --version         Print version
```

By default, directories with `cover.jpg` or `cover.png` will automatically generate cover images. You can customize this with [`~/.config/thud/config.toml`](./examples/config.toml).

## Contributing

Creating your own strategy for thud is easy for both new and experienced Rustaceans. Here are the steps:

1. First add a new function to [`./src/strategy.rs`](./src/strategy.rs)
2. Next add your strategy to `match rule.strategy.as_str()` in [`./src/main.rs`](./src/main.rs)
3. Finally, make a pull request to let anyone be able to use your thumbnailing strategy.

That's it! The possibilities are endless when it comes to how you want your directory thumbnails to look.
