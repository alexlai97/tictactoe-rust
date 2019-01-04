# Command line interface 

## Front page

```
| |_(_) ___| |_ __ _  ___| |_ ___   ___       _ __ _   _ ___| |_ 
| __| |/ __| __/ _` |/ __| __/ _ \ / _ \_____| '__| | | / __| __|
| |_| | (__| || (_| | (__| || (_) |  __/_____| |  | |_| \__ \ |_ 
 \__|_|\___|\__\__,_|\___|\__\___/ \___|     |_|   \__,_|___/\__|

Welcome to tictactoe-rust!

Current configuration: 
  [1]Mode: PVP
  [2]Player1 (User1): "X";
  [3]Player2 (User2): "O"; 
  [4]First: Player1,
  [5]board-size: small; 
  [6]empty-piece: "." 

Available command:
(h)elp (s)tart (q)uit
(1) (2) (3) (4) (5) (6)
```

## Help
```
Game rule:
Two player put their piece turn by turn.
If a player have 3 same pieces on a line 
horizontally, or vertically, or diagonally,
that player wins.

Available command:
(s)tart (b)ack (q)uit
```

### (1) Mode configuration
```
Please enter mode:
  [1] PVP : User1 vs User2
  [2] PVC : User1 vs AI1
  [3] CVC : AI1   vs AI2

Available command:
(1) (2) (3) (b)ack/[Enter] (q)uit
```

### (4) Who plays first
```
Please enter who will play first:
  [1] Player1
  [2] Player2

Available command:
(1) (2) (b)ack/[Enter] (q)uit
```

### (2/3) Player1/2 
```
Configuration for name and piece character:
  [1] Player1 name  ( User1 )
  [2] Player2 name  ( User2 )
  [3] Player1 piece ( "X" )
  [4] Player2 piece ( "O" )

Available command:
(1) (2) (3) (4) (b)ack/[Enter] (q)uit
```

### (5)
```
Only small is available now.
```

### (6)
```
Please enter the piece for Empty piece

Or choose one of the following:
  [sp] " " (space)
  [dt] "." (dot)
```

#### (1/2)
```
Please enter the name for Player1/2
```

#### (3/4)
```
Please enter the piece for Player1/2

Or choose one of the following:
  [bc] ● (black circle)
  [wc] ○ (white circle)
  [bs] ■ (black square)
  [ws] □ (white square)
  [bk] ♚ (black king)
  [wk] ♔ (white king)
  [bq] ♛ (black queen)
  [wq] ♕ (white queen)
  [bp] ♟ (black pawn)
  [wp] ♙ (white pawn)
```

## start
```
------
.|X|O
.|.|X
.|X|O

Where do you what to put your piece, User1/Alex ?

```
### help
```
The borad is 3x3 and each position 
can be represented by a coordinate.

   (0, 0) | (1, 0) | (2, 0)
   ------------------------
   (0, 1) | (1, 1) | (2, 1)
   ------------------------
   (0, 2) | (1, 2) | (2, 2)

Examples:
0 1
2,2
1 , 2
...

Available command:
(b)ack/[Enter] (q)uit
```

### Win (PVP)
```
The winner is User1/Alex.

The matching lines:
  0th row
  1th column
  top-left-bottom-right diagonal

Available command:
(r)estart (f)ontpage (q)uit
```

### Win (PVC)
```
Congratulations, User1/Alex, you win! 

The matching lines:
  0th row
  1th column
  top-left-bottom-right diagonal

Available command:
(r)estart (f)ontpage (q)uit
```

### Lose (PVC)
```
You lost, User1/Alex.

The matching lines:
  0th row
  1th column
  top-left-bottom-right diagonal

Available command:
(r)estart (f)ontpage (q)uit
```

### (CVC)
```
The winner is AI1

The matching lines:
  0th row
  1th column
  top-left-bottom-right diagonal

Available command:
(r)estart (f)ontpage (q)uit
```

# Arguments
tictactoe 
  -h(help)
  -m(mode) [PVP, PVC, CVC] quick start
  -v(version)
