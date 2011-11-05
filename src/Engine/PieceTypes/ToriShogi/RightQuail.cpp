////////////////////////////////////////////////////////////////////////////
// Name:         RightQuail.cpp
// Description:  Implementation for a class that represents a right quail
// Created:      10/20/2004 01:28:20 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "RightQuail.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  RightQuail
//      Method:  RightQuail
// Description:  Constructs an instance of a right quail
//--------------------------------------------------------------------------
RightQuail::RightQuail(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "Q";
   mNames[0] = "Quail (right)"; mNames[1] = "Uzura";
   mDescription =
      "Moves any number of squares north or southwest or 1 square southeast";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   
   // Initialize right quail's static attack patterns
   for (int color = 0; color < 2; color++) {
      mStaticAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         // southeast
         finish = mpBoard->SouthEast(color, start);
         if (finish != Board::OFF_BOARD) {
            mStaticAttackBitboards[color][start].set(finish);
         }
      }  
   }
   
   // Initialize right quail's dynamic attack patterns
   InitAttackPatterns();
}

