////////////////////////////////////////////////////////////////////////////////
// Name:         Bishop.cpp
// Description:  Implementation for a class that represents a bishop
// Created:      May 1, 2004 9:09:12 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Bishop.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Bishop
//      Method:  Bishop
// Description:  Constructs an instance of a bishop
//--------------------------------------------------------------------------
Bishop::Bishop(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "B";
   mNames[0] = "Bishop"; mNames[1] = "Kakugyo";  
   mDescription = "Moves any number of squares diagonally";  

   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the attack patterns
   InitAttackPatterns();
}
