////////////////////////////////////////////////////////////////////////////
// Name:         SideSoldier.cpp
// Description:  Implementation for a class that represents a side soldier
// Created:      09/01/2004 09:06:28 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "SideSoldier.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  SideSoldier
//      Method:  SideSoldier
// Description:  Constructs an instance of a side soldier
//--------------------------------------------------------------------------
SideSoldier::SideSoldier(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "SSo";
   mNames[0] = "Side Soldier"; mNames[1] = "Kohei";
   mDescription =
      "Moves up to 2 squares forward, 1 square backward, or any number of squares horizontally";
  
   // Set the size of the directions and slide limits vectors
   mDirections.resize(DIRECTION_COUNT);
   mSlideLimits.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTH] = &Board::South;
   
   // Establish step limits for each direction
   mSlideLimits[NORTH] = 2;
   mSlideLimits[WEST] = mSize;
   mSlideLimits[EAST] = mSize;
   mSlideLimits[SOUTH] = 1;

   // Initialize side soldier's dynamic attack patterns
   InitAttackPatterns();
}
