////////////////////////////////////////////////////////////////////////////
// Name:         FreeBoar.cpp
// Description:  Implementation for a class that represents a free boar
// Created:      08/31/2004 09:44:11 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "FreeBoar.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  FreeBoar
//      Method:  FreeBoar
// Description:  Constructs an instance of a free boar
//--------------------------------------------------------------------------
FreeBoar::FreeBoar(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "FBo";
   mNames[0] = "Free Boar"; mNames[1] = "Honchu";  
   mDescription = 
      "Moves any number of squares in any direction except vertically";
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the attack patterns
   InitAttackPatterns();
}
