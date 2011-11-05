////////////////////////////////////////////////////////////////////////////
// Name:         WaterBuffalo.cpp
// Description:  Implementation for a class that represents a water buffalo
// Created:      08/28/2004 04:14:32 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "WaterBuffalo.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  WaterBuffalo
//      Method:  WaterBuffalo
// Description:  Constructs an instance of a water buffalo 
//--------------------------------------------------------------------------
WaterBuffalo::WaterBuffalo(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "WBf";
   mNames[0] = "Water Buffalo"; mNames[1] = "Suigyu";  
   mDescription = 
      "Moves up to 2 squares vertically or any number of squares in any other direction";  
  
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
   mSlideLimits[NORTH] = 2;
   mSlideLimits[NORTHEAST] = mSize;
   mSlideLimits[WEST] = mSize;
   mSlideLimits[EAST] = mSize;
   mSlideLimits[SOUTHWEST] = mSize;
   mSlideLimits[SOUTH] = 2;
   mSlideLimits[SOUTHEAST] = mSize;

   // Initialize the attack patterns
   InitAttackPatterns();
}
