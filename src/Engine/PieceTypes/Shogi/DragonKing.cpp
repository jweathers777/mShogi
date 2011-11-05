////////////////////////////////////////////////////////////////////////////////
// Name:         DragonKing.cpp
// Description:  Implementation for a class that represents a dragon king
// Created:      May 1, 2004 9:40:30 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "DragonKing.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  DragonKing
//      Method:  DragonKing
// Description:  Constructs an instance of a dragon king
//--------------------------------------------------------------------------
DragonKing::DragonKing(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
   
   mNotation = "DK";
   mNames[0] = "Dragon King"; mNames[1] = "Ryuo";
   mDescription = 
      "Moves any number of squares orthogonally or 1 square diagonally";  
 
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
 
   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTH] = &Board::South;

   // Initialize dragon king's static attack patterns
   mStaticAttackBitboards = new Bitboard [mSize];

   int square, nextsquare;
   for (square = 0; square < mSize; square++) {
      mStaticAttackBitboards[square].resize(mSize);
      
      // northwest
      nextsquare = mpBoard->NorthWest(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // northeast
      nextsquare = mpBoard->NorthEast(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // southwest
      nextsquare = mpBoard->SouthWest(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // southeast
      nextsquare = mpBoard->SouthEast(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
   }  
   
   // Initialize dragon king's dynamic attack patterns
   InitAttackPatterns();
}

