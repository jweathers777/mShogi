////////////////////////////////////////////////////////////////////////////
// Name:         DrunkElephant.cpp
// Description:  Implementation for a class that represents a drunk elephant
// Created:      08/31/2004 09:27:00 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "DrunkElephant.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  DrunkElephant
//      Method:  DrunkElephant
// Description:  Constructs an instance of a tokin
//--------------------------------------------------------------------------
DrunkElephant::DrunkElephant(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "DE";
   mNames[0] = "Drunk Elephant"; mNames[1] = "Suizo";
   mDescription = 
      "Moves 1 square in any direction except backwards.";

   int finish;
   // Initialize the drunk elephant's attack patterns
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
