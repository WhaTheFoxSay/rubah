# Terms of Use, Copyright Policy, & Privacy Policy

This document outlines the operational principles, data handling practices, technical architecture, and copyright attribution for **Rubah (Ruang Baca Harian)**.

---

## 1. Operational Principles & Application Overview

**Rubah** is an open-source, client-side Terminal User Interface (TUI) RSS/Atom Feed Reader application.

Rubah is designed to interact with publicly accessible RSS 2.0 and Atom feeds published by content providers in accordance with open web syndication standards.

- **Standard Feed Support**: Rubah supports RSS 2.0 and Atom Feed standards as published by content creators.

---

## 2. Open Architecture & Local-First Processing

Rubah operates on a **100% Local-First** architecture without intermediary servers:

```
[ RSS / Atom Publisher ] ──── (Direct Connection) ────> [ Rubah TUI Client ] ────> [ Local SQLite DB ]
```

- **No Intermediary Server**: Rubah does not operate cloud services that aggregate, re-index, or re-distribute articles from publishers through Rubah-owned servers.
- **Client-Side Processing**: All fetching, parsing, and rendering operations take place directly on the user's local machine.
- **Exclusive Local Storage**: Channel configurations, bookmarks, and reading history are stored locally on the user's device (`~/.config/rubah/rubah.db`).

---

## 3. Scope of Use & Technical Operation

Rubah is developed as an RSS/Atom reader for personal and general use based on the following technical principles:

- **Content Integrity**: Rubah displays metadata and content provided in the RSS/Atom Feed as published by the content provider. Rubah does not intentionally alter attribution, author names, publication dates, or original source links.
- **Copyright Integrity**: Rubah does not remove watermarks, attributions, or copyright notices included in the RSS feed.
- **Direct Source Navigation**: Users are provided with a shortcut (`[o]`) to open the original article directly in their official web browser.
- **Functional Equivalence**: Rubah functions similarly to standard RSS/Atom readers such as *Newsboat, Liferea, Feedly, Inoreader, NetNewsWire*, and other feed aggregators.

---

## 4. Content Ownership & Copyright

- **No Ownership Claim**: Rubah claims no ownership rights or licenses over the content displayed. All rights remain exclusively with the respective publishers or copyright holders.
- **Default Feed List**: Default feeds included upon installation are provided solely as examples and for user convenience. Users are free to add, modify, or remove feeds. Default feed selections may change over time in response to publisher policies or project updates.

---

## 5. User Responsibility

Users are responsible for the RSS/Atom feeds they manually add to Rubah. Rubah does not verify the ownership, licensing, or usage policies of feeds added independently by users.

---

## 6. Publisher Rights

All copyrights to articles, images, and other materials belong exclusively to their respective publishers or copyright owners.

Rubah respects publisher rights. If a publisher requests that their RSS Feed be removed from Rubah's default feed list, such requests will be reviewed and honored in good faith.

---

## 7. Feed Removal Request

If you are a publisher or authorized representative and wish to have your RSS Feed removed from Rubah's default feed list, please open an Issue on GitHub or contact the project maintainers.

Valid requests will be reviewed and processed promptly.

---

## 8. Privacy Policy (Zero Telemetry)

**Zero Telemetry**: Rubah does not collect, store, or transmit user identities, IP addresses, or reading activities to any remote servers. All configurations and data reside exclusively on the user's local device.

---

## 9. Disclaimer

This software is provided "as is", without warranty of any kind. The developers of Rubah are not responsible for the accuracy, completeness, or availability of content published by third-party RSS/Atom feed providers.

Availability of feeds depends entirely on respective content providers and may change or terminate at any time without prior notice.

---

## Source Code License

The source code of this application is released under the **GNU General Public License v3.0 (GPL-3.0)**.
