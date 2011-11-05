////////////////////////////////////////////////////////////////////////////
// Name:         FlyingStag.cpp
// Description:  Implementation for a class that represents a flying stag
// Created:      08/31/2004 09:38:15 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "FlyingStag.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  FlyingStag
//      Method:  FlyingStag
// Description:  Constructs an instance of a flying stag
//--------------------------------------------------------------------------
FlyingStag::FlyingStag(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "FS";
   mNames[0] = "Flying Stag"; mNames[1] = "Hiroku";
   mDescription =
      "Moves 1 square in any direction except vertically or any number of squares vertically";
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTH] = &Board::North;
   mDirections[SOUTH] = &Board::South;
   
   // Initialize flying stag's static attack patterns
   mStaticAttackBitboards = new Bitboard [mSize];

   int square, nextsquare;
   for (square = 0; square < mSize; square++) {
      mStaticAttackBitboards[square].resize(mSize);
      
      // northwest
      nextsquare = mpBoard->NorthWest(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // northeast
      nextsquare = mpBoard->NorthEast(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // west
      nextsquare = mpBoard->West(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // east
      nextsquare = mpBoard->East(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // southwest
      nextsquare = mpBoard->SouthWest(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
      // southeast
      nextsquare = mpBoard->SouthEast(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
   }  
   
   // Initialize flying stag's dynamic attack patterns
   InitAttackPatterns();
}

