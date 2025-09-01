# YBC Theme Conversion

Goal: Rebuild `examples/theme/template.html` using only ybc components so the page renders visually identical. Theme colors and tweaks come only from `styles.scss` (no source changes for styling).

## Ground rules
- Use ybc components exclusively; import, don't qualify; no raw Bulma HTML wrappers
- Match layout, sizes, colors, and behaviors (burger, dropdowns, modals)
- Keep interactivity in Rust components; remove template JS/jQuery/GA
- User is running `trunk serve` in background; no need for `cargo build/check` or `trunk serve`; DO run `trunk build` to check syntax at each phase.
- Do one section at a time; verify with user, then mark step as DONE, then commit changes with appropriate commit message.

## Section order (mirrors template)
0. Shell: Container + Columns + Sidebar Menu
1. Header: Navbar (primary) + Hero
2. Elements: Typography, Box, Button, Content, Delete, Form (see #3), Icon, Images, Notifications, Progress, Table, Tag — DONE
3. Forms: Input, Select, Textarea, Checkbox, Radio, File, Field/Control, Addons, Horizontal — DONE
4. Components: Breadcrumb, Card, Dropdown, Level, Media, Menu, Message, Modal, Pagination, Panel, Tabs
5. Variants: Hero color variants, Navbar color variants, Footer

## Mapping: template → ybc
- Layout: `ybc::layout::{Container, Section, Footer, Hero, Level, Media}`; `ybc::columns::{Columns, Column}`
- Navigation: `ybc::components::{Navbar, NavbarItem, NavbarDropdown, Breadcrumb, Pagination, Panel, Tabs, Menu}`
- Elements: `ybc::elements::{Title, Subtitle, Box, Button, Content, Delete, Icon, Image, Notification, Progress, Table, Tag}`
- Forms: `ybc::form::{Field, Control, Input, Select, Textarea, Checkbox, Radio, File}`
- Cards/Modal/Dropdown/Message: `ybc::components::{Card, Modal, Dropdown, Message}`

Missing components: None. The current ybc library contains everything required to reproduce the template, including Columns and all component variations.

## Per‑section checklist (Definition of Done)
- Uses only ybc components (no raw HTML except text and <i> icons inside `Icon`)
- Visual parity: spacing, sizes, colors, and states match
- Behavior parity: burger toggles, dropdown open/close, modal open/close
- Responsive parity: desktop/mobile structure matches Bulma
- No inline styles
- No JS; use yew hook for modal - see `examples/modal/src/main.rs`

## Implementation notes
- Icons: wrap Font Awesome `<i ...>` inside `ybc::elements::Icon`; use size/alignment props when needed
- Navbar: use `Navbar` with `navbrand`, `navstart`, `navend`; burger handled internally; use `NavbarDropdown` for menus; set `padded=true` when the template has container wrapping
- Hero: supply `head`, `body`, `foot`; add `Tabs` to `foot` for hero tabs; set color via Bulma color classes in `classes`
- Tables: toggle `bordered`, `striped`, `narrow`, `fullwidth`, `hoverable`
- Forms: compose `Field` + `Control` with inputs/selects; icons via `Icon` with alignment; use sizes (`is-small`, etc.) via `classes`
- Dropdowns: `hoverable=true` to match hover menus; use `button_html` for trigger contents
- Modal: use provided yew hook in the ybc::Modal component; no JS or jQuery
- Sidebar: build with `Menu`, `MenuLabel`, `MenuList`; place inside `Column is-2`
- Variants: apply Bulma color classes (e.g., `is-primary`) through `classes` props; theme overrides come from `styles.scss`

## Execution plan (bite‑sized)
1) Shell layout: `Container` → `Columns` (left `Column is-2` with `Menu`, right main content)
2) Header: `Navbar` (primary) + `Hero` (title, subtitle, inverted buttons)
3) Typography: `Title`/`Subtitle` sizes; three columns as in template
4) Box + Media: `Box` with `Level` and `Image`
5) Buttons: colors, sizes, states, icons, addons
6) Content: WYSIWYG via `Content`
7) Delete, Notifications, Progress, Table, Tag — DONE
8) Forms: inputs/selects/textarea/checkbox/radio/file, addons, horizontal — DONE
9) Hero (next)
10) Level, Media, Menu, Message
11) Modal
12) Navbar color variants + Hero variants, Tabs, Pagination, Panel
13) Footer

Each step: implement → compare with template → adjust classes/props → commit.

## Low‑priority TODO (post‑template polish)
- Card helpers: optional `CardHeaderTitle`, `CardHeaderIcon` components (sugar over title/icon markup)
- Breadcrumb item helper: optional `BreadcrumbItem` to wrap `li` and handle `is-active`
- Table helpers: optional `TableHead`, `TableBody`, `TableRow`, `TableCell` wrappers for semantics
- Pagination helpers: optional `PaginationLink`/`PaginationListItem` li wrappers (semantics only)
- Navbar helpers: optional `NavbarBrandLogo` shorthand for common brand+logo pattern
- Typographic sugar: `TitleGroup` for paired Title + Subtitle with `is-spaced`
- Example docs: short README in `examples/theme/` explaining how to toggle modal and dropdowns
 - Sticky/fixed sidebar: explore Bulma helpers or minimal CSS to keep left menu fixed while scrolling
- Should all components accept `id` prop?
