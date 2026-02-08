[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua)


# myshenyatko

Terminal UI for managing macOS mouse, trackpad, cursor, and keyboard settings.

Reads and writes macOS `defaults` directly — no System Preferences needed.

## Features

- **Mouse** — tracking speed, acceleration, scroll wheel, natural scroll, swipe navigation
- **Mouse Hardware** — button mode, scroll axes, momentum scroll, tap/swipe gestures (Magic Mouse)
- **Trackpad** — tracking speed, force click, secondary click, corner click, tap to click
- **Trackpad Hardware** — dragging, three-finger drag, pinch, rotate, swipe gestures, click pressure
- **Scroll & Windows** — scrollbar visibility, smooth scrolling, title bar behavior, spring-loaded folders
- **Cursor & Accessibility** — cursor size, scroll-wheel zoom, reduce motion, shake to locate
- **Keyboard** — key repeat rate/delay, press-and-hold, Fn key behavior
- **Text Input** — auto-correct, auto-capitalize, smart quotes/dashes, double-space period

Settings are only shown if your hardware supports them (e.g. Magic Mouse settings are hidden if no mouse domain exists).

Settings that aren't yet configured on your system can still be changed — they start from a sensible default when you first interact with them.

## Install

### From source

```
git clone https://github.com/vhotsyk/myshenyatko.git
cd myshenyatko
make release
cp target/release/myshenyatko /usr/local/bin/
```

### With cargo

```
cargo install --path .
```

## Usage

### Interactive TUI

```
myshenyatko
```

| Key | Action |
|---|---|
| `Tab` / `Shift+Tab` | Switch tabs |
| `Up` / `Down` / `k` / `j` | Navigate settings |
| `Left` / `Right` / `h` / `l` | Adjust value |
| `Space` / `Enter` | Toggle bool / cycle option |
| `r` | Review pending changes |
| `p` | Open profiles |
| `q` | Quit |

Changes are staged — nothing writes to the system until you review and apply.

### CLI

```
myshenyatko dump                    # Print all current settings as JSON
myshenyatko profile list            # List saved profiles
myshenyatko profile apply <name>    # Apply a saved profile
myshenyatko profile export <name>   # Export profile as JSON
myshenyatko profile import <file>   # Import profile from JSON file
```

## Profiles

Save your current settings as a named profile, load it on another machine or after a reset.

Profiles are stored as JSON in `~/.config/myshenyatko/profiles/`.

## Requirements

- macOS
- Rust 2024 edition (1.85+)

## License

[MIT-NORUS](LICENSE)
