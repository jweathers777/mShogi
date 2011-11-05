////////////////////////////////////////////////////////////////////////////
// Name:         LeftQuail.cpp
// Description:  Implementation for a class that represents a left quail
// Created:      10/20/2004 08:25:19 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyleft:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "LeftQuail.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  LeftQuail
//      Method:  LeftQuail
// Description:  Constructs an instance of a left quail
//--------------------------------------------------------------------------
LeftQuail::LeftQuail(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "Q";
   mNames[0] = "Quail (left)"; mNames[1] = "Uzura";
   mDescription =
      "Moves any number of squares north or southeast or 1 square southwest";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTHEAST] = &Board::SouthEast;
   
   // Initialize left quail's static attack patterns
   for (int color = 0; color < 2; color++) {
      mStaticAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         // southwest
         finish = mpBoard->SouthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            mStaticAttackBitboards[color][start].set(finish);
         }
      }  
   }
   
   // Initialize left quail's dynamic attack patterns
   InitAttackPatterns();
}

