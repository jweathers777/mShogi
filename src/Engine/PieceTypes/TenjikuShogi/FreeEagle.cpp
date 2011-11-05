////////////////////////////////////////////////////////////////////////////
// Name:         FreeEagle.cpp
// Description:  Implementation for a class that represents a free eagle
// Created:      08/31/2004 09:47:55 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "FreeEagle.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  FreeEagle
//      Method:  FreeEagle
// Description:  Constructs an instance of a free eagle
//--------------------------------------------------------------------------
FreeEagle::FreeEagle(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "FEg";
   mNames[0] = "Free Eagle"; mNames[1] = "Honju";
   mDescription =
      "Moves like a free king or may jump to the second square in any orthogonal direction";
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTH] = &Board::North;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTH] = &Board::South;
   mDirections[SOUTHEAST] = &Board::SouthEast;
   
   // Initialize free eagle's static attack patterns
   mStaticAttackBitboards = new Bitboard [mSize];

   int square, nextsquare;
   for (square = 0; square < mSize; square++) {
      mStaticAttackBitboards[square].resize(mSize);
      
      // north jump
      nextsquare = mpBoard->North(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         nextsquare = mpBoard->North(0, nextsquare);
         if (nextsquare != Board::OFF_BOARD) {
            mStaticAttackBitboards[square].set(nextsquare);
         }
      }
      // west jump
      nextsquare = mpBoard->West(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         nextsquare = mpBoard->West(0, nextsquare);
         if (nextsquare != Board::OFF_BOARD) {
            mStaticAttackBitboards[square].set(nextsquare);
         }
      }
      // east jump
      nextsquare = mpBoard->East(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         nextsquare = mpBoard->East(0, nextsquare);
         if (nextsquare != Board::OFF_BOARD) {
            mStaticAttackBitboards[square].set(nextsquare);
         }
      }
      // south jump
      nextsquare = mpBoard->South(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         nextsquare = mpBoard->South(0, nextsquare);
         if (nextsquare != Board::OFF_BOARD) {
            mStaticAttackBitboards[square].set(nextsquare);
         }
      }
   }  
   
   // Initialize free eagle's dynamic attack patterns
   InitAttackPatterns();
}
