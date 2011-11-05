////////////////////////////////////////////////////////////////////////////
// Name:         RightChariot.cpp
// Description:  Implementation for a class that represents a right chariot
// Created:      10/25/2004 10:50:21 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#include "RightChariot.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  RightChariot
//      Method:  RightChariot
// Description:  Constructs an instance of a right chariot
//--------------------------------------------------------------------------
RightChariot::RightChariot(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "RCh";
   mNames[0] = "Right Chariot"; mNames[1] = "Usha";
   mDescription =
      "Moves any number of squares north, northeast or southwest or 1 square south";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   
   // Initialize right chariot's static attack patterns
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
   
   // Initialize right chariot's dynamic attack patterns
   InitAttackPatterns();
}

