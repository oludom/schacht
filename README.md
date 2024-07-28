# Schacht chess engine
Yet another attempt at writing a chess engine - cause it's fun :) 

## What about the name?
Bad pun. Writing a chess engine is like a hole that you sink a lot of time into - hence the name 'manhole' that happens to also contain the german word for chess.


```console
   abcdefgh  
  ╔════════╗  
8 ║♖♘♗♔♕♗♘♖║  
7 ║♙♙♙♙♙♙♙♙║  
6 ║▓░▓░▓░▓░║  
5 ║░▓░▓░▓░▓║ White to move  
4 ║▓░▓░▓░▓░║  
3 ║░▓░▓░▓░▓║  
2 ║♟♟♟♟♟♟♟♟║  
1 ║♜♞♝♚♛♝♞♜║  
  ╚════════╝  
   abcdefgh  

```


## The board
- columns: files
- rows: ranks

### BitBoard
The BitBoard implementation assumes that it is
- viewed from whites perspective
- LSB to MSB is right to left on the actual board
- LS Byte to MS Byte is up on the actual board
=> the A1 square is Byte 0 bit 7, the H8 square is Byte 7 bit 0