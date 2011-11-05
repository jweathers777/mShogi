////////////////////////////////////////////////////////////////////////////
// Name:         Phoenix.cpp
// Description:  Implementation for a class that represents a phoenix
// Created:      08/31/2004 10:24:25 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Phoenix.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Phoenix
//      Method:  Phoenix
// Description:  Constructs an instance of a phoenix
//--------------------------------------------------------------------------
Phoenix::Phoenix(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "Ph";
   mNames[0] = "Phoenix"; mNames[1] = "Hoo";
   mDescription =
      "Moves 1 square orthogonally or leaps to the second square diagonally";

   int finish;
   // Initialize the phoenix's attack patterns
   mAttackBitboards = new Bitboard [mSize];
   for (int start = 0; start < mSize; start++) {
      mAttackBitboards[start].resize(mSize);
      
      // northwest jump 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->NorthWest(0, finish);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // northeast jump 
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->NorthEast(0, finish);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
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
      // southwest jump 
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->SouthWest(0, finish);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // southeast jump 
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         finish = mpBoard->SouthEast(0, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }
   }  
}
