# CLI Budget Tracker (aka "where did my money go?")

Tiny Rust command line tool I'm using to bully my own spending.  
You run it from the terminal, tell it what you did with your money, and it yells back at you if you type nonsense.

Right now it’s mainly about **parsing arguments cleanly** and making sure amounts are actually numbers, not “apple”.

---

## What it does (current state)

- Takes a **command** as the first argument:
  - `add`
  - `list`
  - `summary`
- For `add`:
  - expects: `<amount> <category> <note>`
  - checks that `<amount>` is a valid `f64`
  - prints everything back in a nice way
  - screams at you if you:
    - forget an argument
    - type a non-number as amount (`Amount must be a number, got: apple`)
- `list` and `summary` are stubs for now and just print that they were chosen.

It’s basically the skeleton of a real budget tracker:
parsing CLI input properly + a clean place to plug in file/database logic later.

---

## How to run it

You need Rust + Cargo installed.

Clone the repo:

```bash
git clone https://github.com/CodeWithCompound/cli_budget_tracker.git
cd cli_budget_tracker
