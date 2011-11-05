////////////////////////////////////////////////////////////////////////////
// Name:         RookGeneral.cpp
// Description:  Interface for class that represents a rook general
// Created:      08/31/2004 10:41:32 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "RookGeneral.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::map;
using std::make_pair;
using std::string;

//--------------------------------------------------------------------------
//       Class:  RookGeneral
//      Method:  RookGeneral
// Description:  Constructs an instance of a rook general
//--------------------------------------------------------------------------
RookGeneral::RookGeneral(Board* board, int value, int typevalue, int rank,
                             map<int,int>* rankmap)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
   mRank = rank;
      
   mpRankMap = rankmap;
   mpRankMap->insert(make_pair(typevalue, rank));

   mNotation = "RGn";
   mNames[0] = "Rook General"; mNames[1] = "Hisho";  
   mDescription = 
      "Moves any number of squares horizontally or vertically, jumping any pieces of lesser rank to make a capture";

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
