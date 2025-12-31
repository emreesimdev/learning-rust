# Labyrinth

This has been the biggest and hardest project I have built so far. It combines everything I have learned in Rust.

## The Challenge

The beginning of the project was difficult. Even though I knew the logic and what I wanted to do, I struggled to translate that into Rust syntax. That was definitely the hardest part for me.

However, pushing through those difficulties taught me why specific data types are necessary. I experienced firsthand how using `structs` and `enums` makes writing code much easier and cleaner compared to my earlier attempts.

## From 1D to 2D

The project started as a 1-Dimensional line and evolved into a 2-Dimensional grid.

* **Vectors:** I learned how to transform a map from 1D to 2D using nested vectors.
* **Game Logic:** Adding features like Monsters, Traps, and Weapons turned it from a simple pathfinding exercise into a real game.

## The Value of Mistakes

I started writing this without using Enums or advanced logic, which resulted in a lot of unnecessary code. Later, I deleted and refactored almost everything.

Instead of feeling bad about deleting my code, I felt good because I understood *why* I shouldn't write it that way again. I learned that there is a better, easier way.

## Conclusion

It was a painful journey at times, but it was absolutely worth it. I am happy to have completed it and proud to have taken my Rust skills to the next level.

---

## How to Play

**Objective:** Find the exit at the bottom right corner (x:9, y:8).

* **P**: Player
* **#**: Wall
* **.**: Empty Space
* **M**: Monster (Requires Weapon to kill)
* **W**: Weapon (Required to kill Monster)
* **T**: Trap (Deals 2 damage)

**Controls:**
* `w`: Up
* `s`: Down
* `a`: Left
* `d`: Right
* `q`: Quit