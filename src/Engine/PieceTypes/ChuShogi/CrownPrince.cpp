////////////////////////////////////////////////////////////////////////////
// Name:         CrownPrince.cpp
// Description:  Implementation for a class that represents a crown prince
// Created:      08/31/2004 09:24:15 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "CrownPrince.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  CrownPrince
//      Method:  CrownPrince
// Description:  Constructs an instance of a crown prince
//--------------------------------------------------------------------------
CrownPrince::CrownPrince(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "CP";
   mNames[0] = "Crown Prince"; mNames[1] = "Taishi";
   mDescription =
      "Moves like a King";

   int finish;
   // Initialize the crown prince's attack patterns
   mAttackBitboards = new Bitboard [mSize];
   for (int start = 0; start < mSize; start++) {
      mAttackBitboards[start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
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
      // southwest
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // southeast
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
   }  
}
