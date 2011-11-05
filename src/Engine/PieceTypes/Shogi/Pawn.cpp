////////////////////////////////////////////////////////////////////////////////
// Name:         Pawn.cpp
// Description:  Implementation for a class that represents a pawn
// Created:      May 1, 2004 6:08:08 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Pawn.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Pawn
//      Method:  Pawn
// Description:  Constructs an instance of a pawn
//--------------------------------------------------------------------------
Pawn::Pawn(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "P";
   mNames[0] = "Pawn";  mNames[1] = "Fuhyo";
   mDescription = "Moves 1 square forward";

   int finish;

   // Initialize the pawn's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         mAttackBitboards[color][start].resize(mSize);
         
         finish = mpBoard->North(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
      }  
   }
}
