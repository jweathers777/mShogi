////////////////////////////////////////////////////////////////////////////
// Name:         BishopGeneral.cpp
// Description:  Interface for class that represents a bishop general
// Created:      08/31/2004 06:15:28 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "BishopGeneral.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::map;
using std::make_pair;
using std::string;

//--------------------------------------------------------------------------
//       Class:  BishopGeneral
//      Method:  BishopGeneral
// Description:  Constructs an instance of a bishop general
//--------------------------------------------------------------------------
BishopGeneral::BishopGeneral(Board* board, int value, int typevalue, int rank,
                             map<int,int>* rankmap)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
   mRank = rank;
      
   mpRankMap = rankmap;
   mpRankMap->insert(make_pair(typevalue, rank));

   mNotation = "BGn";
   mNames[0] = "Bishop General"; mNames[1] = "Kakusho";  
   mDescription = 
      "Moves any number of squares diagonally, jumping any pieces of lesser rank to make a capture";

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
