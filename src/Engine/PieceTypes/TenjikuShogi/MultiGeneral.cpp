////////////////////////////////////////////////////////////////////////////
// Name:         MultiGeneral.cpp
// Description:  Implementation for a class that represents a multi-general
// Created:      08/31/2004 10:20:27 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "MultiGeneral.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  MultiGeneral
//      Method:  MultiGeneral
// Description:  Constructs an instance of a multi-general 
//--------------------------------------------------------------------------
MultiGeneral::MultiGeneral(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "MGn";
   mNames[0] = "Multi-General"; mNames[1] = "Suisho";
   mDescription = 
      "Moves any number of squares forward or diagonally backwards";
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
  
   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the attack patterns
   InitAttackPatterns();
}
