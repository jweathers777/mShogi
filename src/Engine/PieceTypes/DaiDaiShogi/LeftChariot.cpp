////////////////////////////////////////////////////////////////////////////
// Name:         LeftChariot.cpp
// Description:  Implementation for a class that represents a left chariot
// Created:      10/25/2004 08:54:08 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#include "LeftChariot.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  LeftChariot
//      Method:  LeftChariot
// Description:  Constructs an instance of a left chariot
//--------------------------------------------------------------------------
LeftChariot::LeftChariot(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "LCh";
   mNames[0] = "Left Chariot"; mNames[1] = "Sasha";
   mDescription =
      "Moves any number of squares northwest, north or southeast or 1 square south";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTHEAST] = &Board::SouthEast;
   
   // Initialize left chariot's static attack patterns
   for (int color = 0; color < 2; color++) {
      mStaticAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         // south
         finish = mpBoard->South(color, start);
         if (finish != Board::OFF_BOARD) {
            mStaticAttackBitboards[color][start].set(finish);
         }
      }  
   }
   
   // Initialize left chariot's dynamic attack patterns
   InitAttackPatterns();
}

