////////////////////////////////////////////////////////////////////////////
// Name:         BlindTiger.cpp
// Description:  Implementation for a class that represents a blind tiger
// Created:      08/31/2004 06:22:09 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "BlindTiger.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  BlindTiger
//      Method:  BlindTiger
// Description:  Constructs an instance of a tokin
//--------------------------------------------------------------------------
BlindTiger::BlindTiger(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "BT";
   mNames[0] = "Blind Tiger"; mNames[1] = "Moko";
   mDescription = 
      "Moves 1 square in any direction except forward.";

   int finish;
   // Initialize the blind tiger's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         mAttackBitboards[color][start].resize(mSize);
         
         // northwest 
         finish = mpBoard->NorthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // northeast
         finish = mpBoard->NorthEast(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // west
         finish = mpBoard->West(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // east
         finish = mpBoard->East(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // southwest
         finish = mpBoard->SouthWest(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[color][start].set(finish);
         }
         // south
         finish = mpBoard->South(color, start);
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
