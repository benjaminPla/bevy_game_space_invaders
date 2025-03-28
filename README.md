# Bevy Game - Space Invaders

![output](https://github.com/user-attachments/assets/a2389b3f-034d-4a66-a504-10397a7d01b1)

## Overview

A [Bevy](https://bevyengine.org/) version of Space Invaders, fully developed in Rust.  
Not 100% complete but includes:  
- Five levels  
- Sounds ([jsfxr](https://sfxr.me/))  
- Sprites ([Piskel](https://www.piskelapp.com/))  
- Pause system  
- Animations  
- Collisions  
- Player controls  
- Dynamic texts  

## Structure  

```
.
├── assets
│   ├── audio
│   │   ├── explosion.mp3
│   │   ├── game_over.mp3
│   │   ├── level_completed.mp3
│   │   ├── music.mp3
│   │   └── projectile.mp3
│   ├── fonts
│   │   └── font.ttf
│   └── sprites
│       ├── enemy.png
│       ├── planet.png
│       ├── player.png
│       ├── projectile.png
│       ├── star_big.png
│       └── star_small.png
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── animations.rs
│   ├── assets.rs
│   ├── background.rs
│   ├── collisions.rs
│   ├── constants.rs
│   ├── controls.rs
│   ├── enemy_movement.rs
│   ├── enemy.rs
│   ├── game.rs
│   ├── main.rs
│   ├── player.rs
│   ├── projectiles.rs
│   ├── sound.rs
│   ├── sprites.rs
│   └── texts.rs
└── todo.md
```

## How to Play  

1. Clone the repo  
2. `cargo build`  
3. Run the executable  
