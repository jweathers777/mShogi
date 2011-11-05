////////////////////////////////////////////////////////////////////////////
// Name:         FlyingDragon.cpp
// Description:  Implementation for a class that represents a flying dragon
// Created:      10/23/2004 02:27:30 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "FlyingDragon.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  FlyingDragon
//      Method:  FlyingDragon
// Description:  Constructs an instance of a flying dragon
//--------------------------------------------------------------------------
FlyingDragon::FlyingDragon(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "FD";
   mNames[0] = "Flying Dragon"; mNames[1] = "Hiryu";  
   mDescription = 
      "Moves up to 2 squares the diagonal directions";
  
   // Set the size of the directions and slide limits vectors
   mDirections.resize(DIRECTION_COUNT);
   mSlideLimits.resize(DIRECTION_COUNT);
  
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Establish step limits for each direction
   mSlideLimits[NORTHWEST] = 2;
   mSlideLimits[NORTHEAST] = 2;
   mSlideLimits[SOUTHWEST] = 2;
   mSlideLimits[SOUTHEAST] = 2;

   // Initialize the attack patterns
   InitAttackPatterns();
}
