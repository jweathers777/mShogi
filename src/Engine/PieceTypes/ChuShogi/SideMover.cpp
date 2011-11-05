////////////////////////////////////////////////////////////////////////////
// Name:         SideMover.cpp
// Description:  Implementation for a class that represents a side mover
// Created:      08/31/2004 10:44:39 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "SideMover.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  SideMover
//      Method:  SideMover
// Description:  Constructs an instance of a side mover
//--------------------------------------------------------------------------
SideMover::SideMover(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "SM";
   mNames[0] = "Side Mover"; mNames[1] = "Ogyo";
   mDescription =
      "Moves 1 square vertically or any number of squares horizontally";
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   
   // Initialize side mover's static attack patterns
   mStaticAttackBitboards = new Bitboard [mSize];

   int square, nextsquare;
   for (square = 0; square < mSize; square++) {
      mStaticAttackBitboards[square].resize(mSize);
      
      // north
      nextsquare = mpBoard->North(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // south
      nextsquare = mpBoard->South(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
   }  
   
   // Initialize side mover's dynamic attack patterns
   InitAttackPatterns();
}
