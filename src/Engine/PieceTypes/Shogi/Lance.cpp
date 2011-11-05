////////////////////////////////////////////////////////////////////////////
// Name:         Lance.cpp
// Description:  Implementation for a class that represents a lance
// Created:      08/28/2004 01:32:17 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Lance.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Lance
//      Method:  Lance
// Description:  Constructs an instance of a lance
//--------------------------------------------------------------------------
Lance::Lance(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "RC";
   mNames[0] = "Lance"; mNames[1] = "Kyosha";
   mDescription = "Moves any number of squares vertically forward";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   
   // Initialize the attack patterns
   InitAttackPatterns();
}
