////////////////////////////////////////////////////////////////////////////
// Name:         SavageTiger.cpp
// Description:  Implementation for a class that represents a savage tiger
// Created:      10/25/2004 11:07:02 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "SavageTiger.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  SavageTiger
//      Method:  SavageTiger
// Description:  Constructs an instance of a savage tiger
//--------------------------------------------------------------------------
SavageTiger::SavageTiger(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "ST";
   mNames[0] = "Savage Tiger"; mNames[1] = "Moko";  
   mDescription = 
      "Moves up to 2 squares in any forward direction or backwards";
  
   // Set the size of the directions and slide limits vectors
   mDirections.resize(DIRECTION_COUNT);
   mSlideLimits.resize(DIRECTION_COUNT);
  
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTH] = &Board::North;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTH] = &Board::South;

   // Establish step limits for each direction
   mSlideLimits[NORTHWEST] = 2;
   mSlideLimits[NORTH] = 2;
   mSlideLimits[NORTHEAST] = 2;
   mSlideLimits[SOUTH] = 2;

   // Initialize the attack patterns
   InitAttackPatterns();
}
