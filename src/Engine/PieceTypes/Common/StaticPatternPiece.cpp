////////////////////////////////////////////////////////////////////////////
// Name:         StaticPatternPiece.cpp
// Description:  Implementation for class that represents a piece type whose
//               move can be represented with a static bit pattern
// Created:      08/31/2004 11:27:22 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:22:10 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "StaticPatternPiece.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;

//--------------------------------------------------------------------------
//       Class:  StaticPatternPiece
//      Method:  ~StaticPatternPiece
// Description:  Destroys this instance of a static pattern piece
//--------------------------------------------------------------------------
StaticPatternPiece::~StaticPatternPiece()
{
   for (int color = 0; color < 2; color++) {
      delete [] mAttackBitboards[color];
   }
}

//--------------------------------------------------------------------------
//       Class:  StaticPatternPiece
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool StaticPatternPiece::ThreatensSquare(Piece* piece, int square)
{
   return mAttackBitboards[piece->mColor][piece->mSquare].test(square);
}

//--------------------------------------------------------------------------
//       Class:  StaticPatternPiece
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void StaticPatternPiece::GenerateCaptures(std::vector<Move>& movelist, 
                                          Piece* piece)
{
   int finish;
   Piece* captured;
   Bitboard bitmoves(mSize);
   
   int start = piece->mSquare;
   int color = piece->mColor;

   bitmoves = mAttackBitboards[color][start] &
              mpBoard->mColoredPieces[1-color];
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         captured = mpBoard->mSquares[finish];
         movelist.push_back( Move(piece, start, finish, captured) );
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  StaticPatternPiece
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void StaticPatternPiece::GenerateNonCaptures(std::vector<Move>& movelist, 
                                             Piece* piece)
{
   int finish;
   Bitboard bitmoves(mSize);

   int start = piece->mSquare;
   int color = piece->mColor;

   bitmoves = mAttackBitboards[color][start] &
              mpBoard->mNoPieces;
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         movelist.push_back( Move(piece, start, finish) );
      }
   }
}
