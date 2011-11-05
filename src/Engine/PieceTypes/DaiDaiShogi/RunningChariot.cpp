////////////////////////////////////////////////////////////////////////////
// Name:         RunningChariot.cpp
// Description:  Implementation for a class that represents a running chariot
// Created:      10/25/2004 11:15:51 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "RunningChariot.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  RunningChariot
//      Method:  RunningChariot
// Description:  Constructs an instance of a running chariot
//--------------------------------------------------------------------------
RunningChariot::RunningChariot(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "RnC";
   mNames[0] = "Running Chariot"; mNames[1] = "Sosha";
   mDescription =
      "Moves any number of squares orthogonally or 1 square backwards diagonally";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTH] = &Board::South;
   
   // Initialize running chariot's static attack patterns
   for (int color = 0; color < 2; color++) {
      mStaticAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         // southwest
         finish = mpBoard->SouthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            mStaticAttackBitboards[color][start].set(finish);
         }

         // southeast
         finish = mpBoard->SouthEast(color, start);
         if (finish != Board::OFF_BOARD) {
            mStaticAttackBitboards[color][start].set(finish);
         }

      }  
   }
   
   // Initialize running chariot's dynamic attack patterns
   InitAttackPatterns();
}

