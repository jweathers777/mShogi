////////////////////////////////////////////////////////////////////////////
// Name:         CatSword.cpp
// Description:  Implementation for a class that represents a cat sword
// Created:      10/23/2004 01:42:18 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "CatSword.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  CatSword
//      Method:  CatSword
// Description:  Constructs an instance of a cat sword
//--------------------------------------------------------------------------
CatSword::CatSword(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "CS";
   mNames[0] = "cat sword"; mNames[1] = "Myojin";
   mDescription =
      "Moves 1 square in any diagonal direction";

   int finish;
   // Initialize the cat sword's attack patterns
   mAttackBitboards = new Bitboard [mSize];
   for (int start = 0; start < mSize; start++) {
      mAttackBitboards[start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // southwest
      finish = mpBoard->SouthWest(0, start);
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
