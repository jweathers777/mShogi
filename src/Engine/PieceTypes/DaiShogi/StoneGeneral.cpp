////////////////////////////////////////////////////////////////////////////
// Name:         StoneGeneral.cpp
// Description:  Implementation for a class that represents a Stone General
// Created:      10/23/2004 01:37:36 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "StoneGeneral.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  StoneGeneral
//      Method:  StoneGeneral
// Description:  Constructs an instance of a stone general
//--------------------------------------------------------------------------
StoneGeneral::StoneGeneral(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "ST";
   mNames[0] = "Stone General"; mNames[1] = "Sekisho";   
   mDescription =
      "Moves 1 square diagonally forward";
  
   int finish;
   // Initialize the stone general's attack patterns
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
      }  
   }
}
