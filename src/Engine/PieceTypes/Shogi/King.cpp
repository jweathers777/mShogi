////////////////////////////////////////////////////////////////////////////
// Name:         King.cpp
// Description:  Implementation for a class that represents a king
// Created:      May 1, 2004 08:59:13 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "King.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  King
//      Method:  King
// Description:  Constructs an instance of a king
//--------------------------------------------------------------------------
King::King(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "K";
   mNames[0] = "King"; mNames[1] = "Osho";
   mDescription =
      "Moves 1 square in any direction";

   int finish;
   // Initialize the king's attack patterns
   mAttackBitboards = new Bitboard [mSize];
   for (int start = 0; start < mSize; start++) {
      mAttackBitboards[start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // west
      finish = mpBoard->West(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // east
      finish = mpBoard->East(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // southwest
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // southeast
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
   }  
}
