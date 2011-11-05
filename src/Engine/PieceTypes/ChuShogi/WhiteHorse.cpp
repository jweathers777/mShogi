////////////////////////////////////////////////////////////////////////////
// Name:         WhiteHorse.cpp
// Description:  Implementation for a class that represents a white horse
// Created:      08/28/2004 01:44:17 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "WhiteHorse.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  WhiteHorse
//      Method:  WhiteHorse
// Description:  Constructs an instance of a white horse
//--------------------------------------------------------------------------
WhiteHorse::WhiteHorse(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "WH";
   mNames[0] = "White Horse"; mNames[1] = "Hakku";
   mDescription = 
      "Moves any number of squares vertically or forward diagonally";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
  
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTH] = &Board::North;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTH] = &Board::South;

   // Initialize the attack patterns
   InitAttackPatterns();
}
