////////////////////////////////////////////////////////////////////////////
// Name:         VerticalSoldier.cpp
// Description:  Implementation for a class that represents a vertical soldier
// Created:      09/01/2004 09:20:06 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "VerticalSoldier.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  VerticalSoldier
//      Method:  VerticalSoldier
// Description:  Constructs an instance of a vertical soldier
//--------------------------------------------------------------------------
VerticalSoldier::VerticalSoldier(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "VSo";
   mNames[0] = "Vertical Soldier"; mNames[1] = "Kenhei";
   mDescription =
      "Moves up to 2 squares in either horizontal direction, 1 square backward, or any number of squares forward";
  
   // Set the size of the directions and slide limits vectors
   mDirections.resize(DIRECTION_COUNT);
   mSlideLimits.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTH] = &Board::South;
   
   // Establish step limits for each direction
   mSlideLimits[NORTH] = mSize;
   mSlideLimits[WEST] = 2;
   mSlideLimits[EAST] = 2;
   mSlideLimits[SOUTH] = 1;

   // Initialize vertical soldier's dynamic attack patterns
   InitAttackPatterns();
}
