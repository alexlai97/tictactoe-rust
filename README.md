# tictactoe-rust

## Arch user
```
makepkg PKGBUILD
```

## Build from source file
```
cargo build --release

./target/release/tictactoe-rust
```

## Quick start
just input 
```
start
```
or
```
s
```

## Instruction
in playing page, 
```
X|O|X
.|X|O
O|.|.

```
the coordinates according to the pieces
```
(0, 0) | (1, 0) | (2, 0)
(0, 1) | (1, 1) | (2, 1)
(0, 2) | (1, 2) | (2, 2)
```
so just input
```
0 0
```
or 
```
1,  2
```

## Feature
### Choose your piece to by any thing
Enter the game and input
```
2 3 bc
```
Player1's piece will then be  ● .

You can also put your own.
```
2 3 [your own emoji]
```
### Mode
- PVP: Player vs. Player
- PVC: Player vs. Computer
- CVC: Computer v.s Computer

Let AI1 fight AI2:
```
1 3 s
```
or 
```
(command line) ./tictactoe-rust -m CVC
(stdin) s
```
