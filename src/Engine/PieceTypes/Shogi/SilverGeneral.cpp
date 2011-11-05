////////////////////////////////////////////////////////////////////////////
// Name:         SilverGeneral.cpp
// Description:  Implementation for a class that represents a silver general
// Created:      May 1, 2004 08:21:13 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "SilverGeneral.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  SilverGeneral
//      Method:  SilverGeneral
// Description:  Constructs an instance of a silver general
//--------------------------------------------------------------------------
SilverGeneral::SilverGeneral(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "S";
   mNames[0] = "Silver General"; mNames[1] = "Ginsho";
   mDescription = 
      "Moves 1 square forward or diagonally in any direction";

   int finish;
   // Initialize the silver general's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         mAttackBitboards[color][start].resize(mSize);
         
         // northwest 
         finish = mpBoard->NorthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // north
         finish = mpBoard->North(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // northeast
         finish = mpBoard->NorthEast(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // southwest
         finish = mpBoard->SouthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // southeast
         finish = mpBoard->SouthEast(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
      }  
   }
}
