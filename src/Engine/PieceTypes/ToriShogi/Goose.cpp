////////////////////////////////////////////////////////////////////////////
// Name:         Goose.cpp
// Description:  Implementation for a class that represents a goose
// Created:      10/20/2004 01:21:22 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Goose.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Goose
//      Method:  Goose
// Description:  Constructs an instance of a pawn
//--------------------------------------------------------------------------
Goose::Goose(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "Go";
   mNames[0] = "Goose";  mNames[1] = "Kari";
   //mNames[0] = "Goose";  mNames[1] = "Gan";
   mDescription = "Jumps to the second square in the forward diagonally or backwards";

   int finish;

   // Initialize the goose's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         // northwest jump 
         finish = mpBoard->NorthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            finish = mpBoard->NorthWest(color, start);
            if (finish != Board::OFF_BOARD) {
               mAttackBitboards[start].set(finish);
            }
         }

         // northeast jump 
         finish = mpBoard->NorthEast(color, start);
         if (finish != Board::OFF_BOARD) {
            finish = mpBoard->NorthEast(color, start);
            if (finish != Board::OFF_BOARD) {
               mAttackBitboards[start].set(finish);
            }
         }

         // south jump 
         finish = mpBoard->South(color, start);
         if (finish != Board::OFF_BOARD) {
            finish = mpBoard->South(color, start);
            if (finish != Board::OFF_BOARD) {
               mAttackBitboards[start].set(finish);
            }
         }
      }  
   }
}
