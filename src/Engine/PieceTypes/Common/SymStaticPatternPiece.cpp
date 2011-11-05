////////////////////////////////////////////////////////////////////////////
// Name:         SymStaticPatternPiece.cpp
// Description:  Interface for class that represents a piece type whose
//               move can be represented with a symmetric static bit pattern
// Created:      08/31/2004 11:55:19 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:22:10 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "SymStaticPatternPiece.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;

//--------------------------------------------------------------------------
//       Class:  SymStaticPatternPiece
//      Method:  ~SymStaticPatternPiece
// Description:  Destroys this instance of a static pattern piece
//--------------------------------------------------------------------------
SymStaticPatternPiece::~SymStaticPatternPiece()
{
   delete [] mAttackBitboards;
}

//--------------------------------------------------------------------------
//       Class:  SymStaticPatternPiece
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool SymStaticPatternPiece::ThreatensSquare(Piece* piece, int square)
{
   return mAttackBitboards[piece->mSquare].test(square);
}

//--------------------------------------------------------------------------
//       Class:  SymStaticPatternPiece
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void SymStaticPatternPiece::GenerateCaptures(std::vector<Move>& movelist, 
                                          Piece* piece)
{
   int finish;
   Piece* captured;
   Bitboard bitmoves(mSize);
   
   int start = piece->mSquare;

   bitmoves = mAttackBitboards[start] &
              mpBoard->mColoredPieces[1 - piece->mColor];
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         captured = mpBoard->mSquares[finish];
         movelist.push_back( Move(piece, start, finish, captured) );
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  SymStaticPatternPiece
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void SymStaticPatternPiece::GenerateNonCaptures(std::vector<Move>& movelist, 
                                             Piece* piece)
{
   int finish;
   Bitboard bitmoves(mSize);

   int start = piece->mSquare;

   bitmoves = mAttackBitboards[start] &
              mpBoard->mNoPieces;
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         movelist.push_back( Move(piece, start, finish) );
      }
   }
}
