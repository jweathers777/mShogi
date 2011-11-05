////////////////////////////////////////////////////////////////////////////
// Name:         ViolentOx.cpp
// Description:  Implementation for a class that represents a violent ox
// Created:      10/23/2004 01:49:43 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "ViolentOx.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  ViolentOx
//      Method:  ViolentOx
// Description:  Constructs an instance of a violent ox
//--------------------------------------------------------------------------
ViolentOx::ViolentOx(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "VO";
   mNames[0] = "Violent Ox"; mNames[1] = "Mogyu";  
   mDescription = 
      "Moves up to 2 squares the orthogonal directions";
  
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
   mSlideLimits[WEST] = 2;
   mSlideLimits[EAST] = 2;
   mSlideLimits[SOUTH] = 2;

   // Initialize the attack patterns
   InitAttackPatterns();
}
