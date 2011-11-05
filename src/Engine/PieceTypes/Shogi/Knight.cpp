////////////////////////////////////////////////////////////////////////////
// Name:         Knight.cpp
// Description:  Implementation for class that represents a Knight
// Created:      08/28/2004 01:04:48 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Knight.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Knight
//      Method:  Knight
// Description:  Constructs an instance of a knight
//--------------------------------------------------------------------------
Knight::Knight(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
   mNotation = "N";
   mNames[0] = "Knight"; mNames[1] = "Keima";
   mDescription =
      "Moves 2 squares vertically forward and then 1 square horizontally, leaping as it goes";

   int finish;
   // Initialize the knight's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         mAttackBitboards[color][start].resize(mSize);
         
         // +-----+-----+-----+
         // |  F  |     |     |    S=start
         // +-----+-----+-----+    E=finish
         // |     |     |     |
         // +-----+-----+-----+
         // |     |  S  |     |
         // +-----+-----+-----+

         finish = mpBoard->North(color, start);
         if (finish != Board::OFF_BOARD) {
            finish = mpBoard->North(color, finish);
            if (finish != Board::OFF_BOARD) {
               finish = mpBoard->West(color, finish);
               if (finish != Board::OFF_BOARD) {
                  mAttackBitboards[color][start].set(finish);
               }
            }
         }

         // +-----+-----+-----+
         // |     |     |  F  |    S=start
         // +-----+-----+-----+    E=finish
         // |     |     |     |
         // +-----+-----+-----+
         // |     |  S  |     |
         // +-----+-----+-----+

         finish = mpBoard->North(color, start);
         if (finish != Board::OFF_BOARD) {
            finish = mpBoard->North(color, finish);
            if (finish != Board::OFF_BOARD) {
               finish = mpBoard->East(color, finish);
               if (finish != Board::OFF_BOARD) {
                  mAttackBitboards[color][start].set(finish);
               }
            }
         }
      }  
   }
}
