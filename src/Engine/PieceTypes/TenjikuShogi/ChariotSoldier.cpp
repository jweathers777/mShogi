////////////////////////////////////////////////////////////////////////////
// Name:         ChariotSoldier.cpp
// Description:  Implementation for a class that represents a chariot soldier
// Created:      08/31/2004 06:25:48 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "ChariotSoldier.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  ChariotSoldier
//      Method:  ChariotSoldier
// Description:  Constructs an instance of a chariot soldier
//--------------------------------------------------------------------------
ChariotSoldier::ChariotSoldier(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "ChS";
   mNames[0] = "Chariot Soldier"; mNames[1] = "Shahei";  
   mDescription = 
      "Moves up to 2 squares in either horizontal direction or any number of squares in any other direction";
  
   // Set the size of the directions and slide limits vectors
   mDirections.resize(DIRECTION_COUNT);
   mSlideLimits.resize(DIRECTION_COUNT);
  
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTH] = &Board::North;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTH] = &Board::South;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Establish step limits for each direction
   mSlideLimits[NORTHWEST] = mSize;
   mSlideLimits[NORTH] = mSize;
   mSlideLimits[NORTHEAST] = mSize;
   mSlideLimits[WEST] = 2;
   mSlideLimits[EAST] = 2;
   mSlideLimits[SOUTHWEST] = mSize;
   mSlideLimits[SOUTH] = mSize;
   mSlideLimits[SOUTHEAST] = mSize;

   // Initialize the attack patterns
   InitAttackPatterns();
}
