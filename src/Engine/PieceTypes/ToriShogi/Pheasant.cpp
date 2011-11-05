////////////////////////////////////////////////////////////////////////////
// Name:         Pheasant.cpp
// Description:  Implementation for a class that represents a pheasant
// Created:      10/20/2004 12:02:32 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Pheasant.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Pheasant
//      Method:  Pheasant
// Description:  Constructs an instance of a pawn
//--------------------------------------------------------------------------
Pheasant::Pheasant(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "Pt";
   mNames[0] = "Pheasant";  mNames[1] = "Kiji";
   mDescription = "Jumps to the second square in the forward direction or backwards 1 space diagonally";

   int finish;

   // Initialize the pheasant's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         // north jump 
         finish = mpBoard->North(color, start);
         if (finish != Board::OFF_BOARD) {
            finish = mpBoard->North(color, start);
            if (finish != Board::OFF_BOARD) {
               mAttackBitboards[start].set(finish);
            }
         }

         // southwest
         finish = mpBoard->SouthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
         
         // southeast
         finish = mpBoard->SouthEast(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }  
   }
}
