////////////////////////////////////////////////////////////////////////////
// Name:         HowlingDog.cpp
// Description:  Implementation for a class that represents a howling dog
// Created:      10/23/2004 02:40:49 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "HowlingDog.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  HowlingDog
//      Method:  HowlingDog
// Description:  Constructs an instance of a howling dog
//--------------------------------------------------------------------------
HowlingDog::HowlingDog(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "HD";
   mNames[0] = "Howling Dog"; mNames[1] = "Kiken";
   mDescription =
      "Moves any number of squares forward or 1 square backward";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   
   // Initialize howling dog's static attack patterns
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
   
   // Initialize howling dog's dynamic attack patterns
   InitAttackPatterns();
}

