////////////////////////////////////////////////////////////////////////////
// Name:         VerticalMover.cpp
// Description:  Implementation for a class that represents a vertical mover
// Created:      08/31/2004 10:50:00 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "VerticalMover.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  VerticalMover
//      Method:  VerticalMover
// Description:  Constructs an instance of a vertical mover
//--------------------------------------------------------------------------
VerticalMover::VerticalMover(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "VM";
   mNames[0] = "Vertical Mover"; mNames[1] = "Kengyo";
   mDescription =
      "Moves 1 square horizontally or any number of squares vertically";
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTH] = &Board::South;
   
   // Initialize vertical mover's static attack patterns
   mStaticAttackBitboards = new Bitboard [mSize];

   int square, nextsquare;
   for (square = 0; square < mSize; square++) {
      mStaticAttackBitboards[square].resize(mSize);
      
      // west
      nextsquare = mpBoard->West(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // east
      nextsquare = mpBoard->East(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
   }  
   
   // Initialize vertical mover's dynamic attack patterns
   InitAttackPatterns();
}
