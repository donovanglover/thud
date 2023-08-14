# thud

*thud*, short for **thu**mbnail **d**irectory, is a tool that lets you generate directory thumbnails from images inside them. Useful for a Dolphin/KDE-like experience with GTK-based file browsers that don't show the contents of directories by default.

thud can be customized with *strategies* that reside in `~/.config/thud/config.toml`. By default, it creates thumbnails from `cover.{png,jpg}` files.

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

thud should work out of the box. Read the man pages for how to customize thud with `~/.config/thud/config.toml`.

## Contributing

Anyone can add their own strategies to thud by forking the repository and making a pull request.
