////////////////////////////////////////////////////////////////////////////
// Name:         EvilWolf.cpp
// Description:  Implementation for a class that represents a evil wolf
// Created:      10/23/2004 02:23:47 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "EvilWolf.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  EvilWolf
//      Method:  EvilWolf
// Description:  Constructs an instance of a evil wolf
//--------------------------------------------------------------------------
EvilWolf::EvilWolf(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "EW";
   mNames[0] = "Evil Wolf"; mNames[1] = "Akuro";   
   mDescription =
      "Moves 1 square in any forward direction or horizontal direction";
  
   int finish;
   // Initialize the evil wolf's attack patterns
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
      }  
   }
}
