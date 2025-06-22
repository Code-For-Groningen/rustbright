# rustbright
Brightspace UI (specifically for TAs) written in Rust.

## Resources
- [Gtk-rs](https://gtk-rs.org/)
- [Gnome design repository](https://gitlab.gnome.org/Teams/Design/)
- [Gtk-4 docs](https://docs.gtk.org/gtk4/)

## Design
Inspired by Gnome's design principles.

### Authentication
<!-- Use tauri for a webview to capture cookies -->
<!-- tauri logo -->
![Tauri Logo](https://tauri.app/_astro/logo.DCjQDXhk.svg)
(might be a bit overkill)
```mermaid
flowchart TD
    A[User] -->|Login| B[Headless Webview]
    B --> C[Cookie Storage]
    C --> D[API Requests]
    D --> E[Brightspace API]
    E --> F[Data Response]
    F --> G[Display Data]
```

### UI
View course content, grades, and assignments in a user-friendly interface.

#### Colors

| Color Name | Hex Code | Preview | Usage |
|------------|----------|---------|-------|
| University Red | `#CC0000` | ![#CC0000](https://dummyimage.com/20x20/CC0000/CC0000) | Primary brand color (University of Groningen) |
| Brightspace Dark Red | `#DC002D` | ![#7C021C](https://dummyimage.com/20x20/DC002D/DC002D) | Secondary accent color |
| Brightspace Blue | `#006FBF` | ![#275374](https://dummyimage.com/20x20/006FBF/006FBF) | Background color for Brightspace elements |
| Brightspace Light Blue | `#0095E9` | ![#0095E9](https://dummyimage.com/20x20/0095E9/0095E9) | Highlight color for links and buttons |
| Brightspace Orange | `#E87511` | ![#E87511](https://dummyimage.com/20x20/E87511/E87511) | Accent color for notifications and alerts |

![Color Palette](assets/ColorPalette.png)

#### Fonts
| Font Name | Usage |
|------------|-------|
| Cantarell | Primary font for headings and titles (faithful to Gnome) |
| Roboto | Secondary font for UI elements and buttons |

Very modern design!

### Mockups


