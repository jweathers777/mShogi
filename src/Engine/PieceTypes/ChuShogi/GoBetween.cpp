////////////////////////////////////////////////////////////////////////////
// Name:         GoBetween.cpp
// Description:  Implementation for a class that represents a Go-Between
// Created:      08/28/2004 12:52:46 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "GoBetween.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  GoBetween
//      Method:  GoBetween
// Description:  Constructs an instance of a go-between
//--------------------------------------------------------------------------
GoBetween::GoBetween(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "GB";
   mNames[0] = "Go-Between"; mNames[1] = "Chunin";
   mDescription =  "Moves 1 square vertically";

   int finish;
   // Initialize the go-between's attack patterns
   mAttackBitboards = new Bitboard [mSize];
   for (int start = 0; start < mSize; start++) {
      mAttackBitboards[start].resize(mSize);
      
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[start].set(finish);
      }
   }  
}
