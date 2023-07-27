## Known issues

Most file browsers update thumbnails when the modified date changes, as per the [Thumbnail Managing Standard](https://specifications.freedesktop.org/thumbnail-spec/thumbnail-spec-latest.html#MODIFICATIONS). This is good when thumbnailing individual files, but becomes a problem when thumbnailing directories since their modification date [changes](https://stackoverflow.com/questions/3620684/directory-last-modified-date/3620704#3620704) when the file structure inside them changes, such as when you rename one of the files.

What this means is that it's up to the file browser to gracefully handle this unusual behavior of the modified date being updated but the thumbnail being the same.

### Thunar

- Very fast, although work is being done to [make it faster](https://gitlab.xfce.org/xfce/tumbler/-/issues/1).
- Loads large directories well.
- Uses the default icon when loading thumbnails, so directories will use the folder icon while they load.
- No infinite loading bug, and no visual indicator that the thumbnail is being regenerated, possibly because it uses the existing folder icon instead of `image-loading.svg`.

Cons of using thunar:

- Thumbnails aren't generated until you scroll to them, which means that you can't open a large directory and have all thumbnails be automatically generated.
- Thumbnail sizes for non-directories may be less polished than Nautilus.

When using thunar, make sure you have the [tumbler](https://gitlab.xfce.org/xfce/tumbler) package installed. [Source](https://superuser.com/questions/258633/why-is-thunar-not-creating-and-showing-thumbnails-of-images/259471#259471).

### Caja

- Fast.
- Has a cascading effect on first load, followed by loading everything at once when revisiting that directory.
- Uses `image-loading.svg` while directory thumbnail loads.
- Visual indicator that the thumbnail is being regenerated, but no infinite loading bug.

### Nautilus

- Fast, but thumbnail generation was intentionally slowed down based on the source code. See [lines 82-84](https://gitlab.gnome.org/GNOME/nautilus/-/blob/794931998cb27b7ca94651c72300a5ed167a8951/src/nautilus-thumbnails.c#L82-84) and [147-171](https://gitlab.gnome.org/GNOME/nautilus/-/blob/794931998cb27b7ca94651c72300a5ed167a8951/src/nautilus-thumbnails.c#L147-171) of `nautilus-thumbnails.c`. The issue [has been discussed](https://gitlab.gnome.org/GNOME/nautilus/-/issues/856) and [several](https://gitlab.gnome.org/GNOME/nautilus/-/merge_requests/660) [solutions](https://gitlab.gnome.org/GNOME/nautilus/-/merge_requests/700) have been proposed.
- ~~Has a cascading effect when loading large directories. See: [Gnome Files is unusable with big folders](https://gitlab.gnome.org/GNOME/nautilus/-/issues/1967) and [Folder contents scroll down as thumbnails are generated in icon view](https://gitlab.gnome.org/GNOME/nautilus/-/issues/1720).~~ **[Fixed in Nautilus 43](https://gitlab.gnome.org/GNOME/nautilus/-/issues/1720#note_1483977)**.
- Uses `image-loading.svg` while directory thumbnail loads.
- ~~Has infinite loading bug after changing the file structure of the current directory and moving up one directory. A [known issue](https://gitlab.gnome.org/GNOME/nautilus/-/issues/1887) since June 2021.~~ **Seems to be fixed in Nautilus 43, however directory thumbnails do not automatically update like Thunar**.

### Nemo

- Slow to generate thumbnails.
- Has a cascading effect when loading large directories.
- Uses `image-loading.svg` while directory thumbnail loads.
- Has infinite loading bug after changing the file structure of the current directory and moving up one directory. Similar to [this bug report](https://github.com/linuxmint/nemo/issues/2736), but triggered differently.

### Summary

For ease of access, here's a summary of above.

| File browser | Fast? | No cascade?  | Folder placeholders? | No infinite loading bug? |
| --- | :---: | :---: | :---: | :---: |
| **Thunar** | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| **Caja** | :white_check_mark: | After first load | :x: | :white_check_mark: |
| **Nautilus** | :white_check_mark: | After first load (Nautilus 43+) | :x: | Nautilus 43+ |
| **Nemo** | :x: | :x: | :x: | :x: |

Recommendation: Thunar offers a pleasurable experience. It is very fast, there is no cascading effect, and covers are automatically updated as you change them. Nautilus also works, but you will have to manually delete old thumbnails to update them.
