# Board Representation

## Piece Centric representation:
- Make an array of size 64. Each item of this array will represent a piece.
- Each item of this array will be a struct with the following values:
1. alive: bool -> Is the piece alive (true) or has it been captured (false).
2. type: enum -> The type of piece (pawn, rook, horse, bishop, knight, queen or king).
3. position: enum -> position of the piece on the board
4. color: bool -> true if white, false if black.

## Board Centric representation:
- Make an array of arrays (8x8)
- Each cell of the board has the following information:
1. piece: Option<Piece> -> Option that contains the struct piece that is in this cell.
- The piece struct contains the type and color of the piece.