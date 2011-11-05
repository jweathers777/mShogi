////////////////////////////////////////////////////////////////////////////
// Name:         GreatGeneral.cpp
// Description:  Interface for class that represents a great general
// Created:      08/31/2004 10:00:48 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "GreatGeneral.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::map;
using std::make_pair;
using std::string;

//--------------------------------------------------------------------------
//       Class:  GreatGeneral
//      Method:  GreatGeneral
// Description:  Constructs an instance of a great general
//--------------------------------------------------------------------------
GreatGeneral::GreatGeneral(Board* board, int value, int typevalue, int rank,
                             map<int,int>* rankmap)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
   mRank = rank;
      
   mpRankMap = rankmap;
   mpRankMap->insert(make_pair(typevalue, rank));

   mNotation = "GGn";
   mNames[0] = "Great General"; mNames[1] = "Daisho";  
   mDescription = 
      "Moves any number of squares in any direction, jumping any pieces of lesser rank to make a capture";

   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTH] = &Board::North;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTH] = &Board::South;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the attack patterns
   InitAttackPatterns();
}
