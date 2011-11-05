////////////////////////////////////////////////////////////////////////////////
// Name:         DragonHorse.cpp
// Description:  Implementation for a class that represents a dragon horse
// Created:      May 1, 2004 9:40:30 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "DragonHorse.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  DragonHorse
//      Method:  DragonHorse
// Description:  Constructs an instance of a dragon horse
//--------------------------------------------------------------------------
DragonHorse::DragonHorse(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
 
   mNotation = "DH";
   mNames[0] = "Dragon Horse"; mNames[1] = "Ryume";
   mDescription =
      "Moves any number of squares diagonally or 1 square orthogonally";  
  
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);

   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTHEAST] = &Board::SouthEast;
   
   // Initialize dragon horse's static attack patterns
   mStaticAttackBitboards = new Bitboard [mSize];

   int square, nextsquare;
   for (square = 0; square < mSize; square++) {
      mStaticAttackBitboards[square].resize(mSize);
      
      // north 
      nextsquare = mpBoard->North(0, square);
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
      // south
      nextsquare = mpBoard->South(0, square);
      if (nextsquare != Board::OFF_BOARD) {
         mStaticAttackBitboards[square].set(nextsquare);
      }
   }  
   
   // Initialize dragon horse's dynamic attack patterns
   InitAttackPatterns();
}

