////////////////////////////////////////////////////////////////////////////
// Name:         Swallow.cpp
// Description:  Implementation for a class that represents a pheasant
// Created:      10/20/2004 01:17:52 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Swallow.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Swallow
//      Method:  Swallow
// Description:  Constructs an instance of a pawn
//--------------------------------------------------------------------------
Swallow::Swallow(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "Sw";
   mNames[0] = "Swallow";  mNames[1] = "Tsubame";
   mDescription = "Moves 1 step forward";

   int finish;

   // Initialize the swallow's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         // north
         finish = mpBoard->North(color, start);
         if (finish != Board::OFF_BOARD) {
            mAttackBitboards[start].set(finish);
         }
      }  
   }
}
