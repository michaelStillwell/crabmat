# Crabmat

Terminal based kanban bored inspired by [kabmat](https://github.com/PlankCipher/kabmat).

---

## Usage

Run `crabmat` in the terminal. If no file is supplied then a file named `kanban` will be created.
And if the file doesn't have a title or is empty, you will be prompted to enter a title for the
new kanban board.

---

### Main Screen

| Key | Action |
|---|---|
| h | move column focus left |
| l | move column focus right |
| j | move card focus down |
| k | move card focus up |
| \<C-h\> | move column right |
| \<C-l\> | move column left |
| H | move card left |
| L | move card right |
| J | move card down |
| K | move card up |
| d | delete card |
| D | delete column |
| q | quit |

### Edit/New Column

| Key | Action |
|---|---|
| s/Enter | save column |
| q/Esc | exit without saving |

### Edit/New Card

| Key | Action |
|---|---|
| s | save card |
| q/Esc | exit without saving |
| \<C-j\> | edit description |
| \<C-k\> | edit title |
| Enter | when editing the title, edit description |

