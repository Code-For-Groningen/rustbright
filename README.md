# rustbright
Brightspace UI (specifically for TAs) written in Rust.

## Resources
- [Gtk-rs](https://gtk-rs.org/)
- [Gnome design repository](https://gitlab.gnome.org/Teams/Design/)
- [Gtk-4 docs](https://docs.gtk.org/gtk4/)
- [Brightspace API documentation](https://docs.valence.desire2learn.com/reference.html)

## Design
Inspired by Gnome's design principles.

### Authentication
![Tauri Logo](https://tauri.app/_astro/logo.DCjQDXhk.svg)
(might be a bit overkill)
```mermaid
flowchart LR
    A[Login] --> B[Headless Browser] --> C[Brightspace]
    C --> D[2FA] --> B --> E[Cookies]
    
    style A fill:#006FBF,color:#fff
    style C fill:#DC002D,color:#fff
    style D fill:#0095E9,color:#fff
```
#### Mockup
<div align="center">

![Authentication Mockup](assets/authentication.png)

</div>

### Application Flow
```mermaid
flowchart LR
    A[Cookies] --> B[API] --> C[Parser] --> D[GTK UI] --> E[Grading] --> B
    
    style D fill:#0095E9,color:#fff
    style E fill:#E87511,color:#fff
```

#### Main UI Mockup

<div align="center">

![Main UI Mockup](assets/flow.png)

</div>

### UI Styling
View course content, grades, and assignments in a user-friendly interface.

#### Colors
![Color Palette](assets/ColorPalette.png)

#### Fonts
| Font Name | Usage |
|------------|-------|
| Cantarell | Primary font for headings and titles (faithful to Gnome) |
| Roboto | Secondary font for UI elements and buttons |

Very modern design!

### File Structure
```
src/
├── main.rs          # Main entry point
├── auth/         # Authentication logic
|  ├── mod.rs
|  └── login.rs
├── api/          # API interaction
|  ├── mod.rs
|  └── client.rs
```


## TODOs
- [ ] Review assignments and grades (maybe outside of scope for now)
- [ ] Actually implement shit



