////////////////////////////////////////////////////////////////////////////
// Name:         Kylin.cpp
// Description:  Implementation for a class that represents a kylin
// Created:      08/31/2004 10:15:29 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Kylin.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Kylin
//      Method:  Kylin
// Description:  Constructs an instance of a kylin
//--------------------------------------------------------------------------
Kylin::Kylin(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "Ky";
   mNames[0] = "Kylin"; mNames[1] = "Kirin";
   mDescription =
      "Moves 1 square diagonally or leaps to the second square orthogonally";

   int finish;
   // Initialize the kylin's attack patterns
   mAttackBitboards = new Bitboard [mSize];
   for (int start = 0; start < mSize; start++) {
      mAttackBitboards[start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // north jump
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->North(0, finish);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // west jump
      finish = mpBoard->West(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->West(0, finish);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }
      // east jump
      finish = mpBoard->East(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->East(0, finish);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }
      // southwest
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // south jump
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->South(0, finish);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }
      // southeast
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
   }  
}
