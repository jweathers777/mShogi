////////////////////////////////////////////////////////////////////////////
// Name:         Dog.cpp
// Description:  Implementation for class that represents a Dog
// Created:      08/28/2004 12:58:51 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Dog.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Dog
//      Method:  Dog
// Description:  Constructs an instance of a dog
//--------------------------------------------------------------------------
Dog::Dog(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "Dg";
   mNames[0] = "Dog";  mNames[1] = "Inu";
   mDescription = 
      "Moves 1 square vertically forward or 1 square diagonally backwards";

   int finish;
   // Initialize the dog's attack patterns
   for (int color = 0; color < 2; color++) {
      mAttackBitboards[color] = new Bitboard [mSize];
      for (int start = 0; start < mSize; start++) {
         mAttackBitboards[color][start].resize(mSize);
         
         // north
         finish = mpBoard->North(color, start);
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

