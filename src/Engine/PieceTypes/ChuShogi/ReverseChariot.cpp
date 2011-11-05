////////////////////////////////////////////////////////////////////////////
// Name:         ReverseChariot.cpp
// Description:  Implementation for a class that represents a reverse chariot
// Created:      08/28/2004 01:24:39 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "ReverseChariot.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  ReverseChariot
//      Method:  ReverseChariot
// Description:  Constructs an instance of a reverse chariot
//--------------------------------------------------------------------------
ReverseChariot::ReverseChariot(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "RC";
   mNames[0] = "Reverse Chariot"; mNames[1] = "Hansha";  
   mDescription = "Moves any number of squares vertically";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTH] = &Board::South;

   // Initialize the attack patterns
   InitAttackPatterns();
}
