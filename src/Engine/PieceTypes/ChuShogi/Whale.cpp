////////////////////////////////////////////////////////////////////////////
// Name:         Whale.cpp
// Description:  Implementation for a class that represents a whale
// Created:      08/28/2004 04:09:24 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Whale.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Whale
//      Method:  Whale
// Description:  Constructs an instance of a whale 
//--------------------------------------------------------------------------
Whale::Whale(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "Wh";
   mNames[0] = "Whale"; mNames[1] = "Keigei";
   mDescription = 
      "Moves any number of squares vertically or backward diagonally";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
  
   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTH] = &Board::South;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the attack patterns
   InitAttackPatterns();
}
