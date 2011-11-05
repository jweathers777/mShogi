////////////////////////////////////////////////////////////////////////////////
// Name:         Rook.cpp
// Description:  Implementation for a class that represents a rook
// Created:      May 2, 2004 12:01:23 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Rook.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Rook
//      Method:  Rook
// Description:  Constructs an instance of a rook
//--------------------------------------------------------------------------
Rook::Rook(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "R";
   mNames[0] = "Rook"; mNames[1] = "Hisha";  
   mDescription = "Moves any number of squares orthogonally";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTH] = &Board::South;

   // Initialize the attack patterns
   InitAttackPatterns();
}
