////////////////////////////////////////////////////////////////////////////
// Name:         ViolentBear.cpp
// Description:  Implementation for a class that represents a violent bear
// Created:      10/25/2004 11:26:59 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "ViolentBear.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  ViolentBear
//      Method:  ViolentBear
// Description:  Constructs an instance of a violent bear
//--------------------------------------------------------------------------
ViolentBear::ViolentBear(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "ST";
   mNames[0] = "Violent Bear"; mNames[1] = "Moyu";  
   mDescription = 
      "Moves up to 2 squares diagonally forward or 1 square horizontally";
  
   // Set the size of the directions and slide limits vectors
   mDirections.resize(DIRECTION_COUNT);
   mSlideLimits.resize(DIRECTION_COUNT);
  
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;

   // Establish step limits for each direction
   mSlideLimits[NORTHWEST] = 2;
   mSlideLimits[NORTHEAST] = 2;
   mSlideLimits[WEST] = 1;
   mSlideLimits[EAST] = 1;

   // Initialize the attack patterns
   InitAttackPatterns();
}
