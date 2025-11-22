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
- add → fully working
  
validates amount |
 writes entries to budget.csv |
 prints nice formatted output

- list → now fully implemented

opens budget.csv |
 prints every entry cleanly |
 basically a tiny file viewer for my financial crimes

- summary → half-built but alive
  
reads the whole CSV file into a string |
 total spend + category breakdown coming soon |
 right now it just prints the raw file so i can see what i’m working with 

---

## How to run it

You need Rust + Cargo installed.

Clone the repo:

```bash
git clone https://github.com/CodeWithCompound/cli_budget_tracker.git
cd cli_budget_tracker
