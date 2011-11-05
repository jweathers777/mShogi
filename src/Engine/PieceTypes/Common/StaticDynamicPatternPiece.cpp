////////////////////////////////////////////////////////////////////////////
// Name:         StaticDynamicPatternPiece.cpp
// Description:  Interface for class that represents a piece type
//               whose move can be represented with a static bit pattern
//               a dynamic bit pattern
// Created:      10/20/2004 08:02:36 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "StaticDynamicPatternPiece.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;

//------------------------------------------------------------------------
//       Class:  StaticDynamicPatternPiece
//      Method:  InitAttackPatterns
// Description:  Initialize the attack pattern bitboards for moves
//               that involve dynamic bit patterns
//------------------------------------------------------------------------
void StaticDynamicPatternPiece::InitAttackPatterns()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();

   // Set the size of each color's attack boards vector
   int color;
   for (color = 0; color < 2; color++) {
      mAttackBitboards[color].resize(direction_count);
   }
   
   int square, nextsquare;
   for (color = 0; color < 2; color++) {
      // Create new bitboard arrays for each color and direction
      for (direction = 0; direction < direction_count; direction++) {
         mAttackBitboards[color][direction] = new Bitboard [mSize];
      }
      
      // Resize each bitboard to fit the current board size
      for (square = 0; square < mSize; square++) {
         for (direction = 0; direction < direction_count; direction++) {
            mAttackBitboards[color][direction][square].resize(mSize);
         }
      }  
      
      // Fill each bitboard with the appropriate bit
      // pattern based upon the direction of movement
      for (square = 0; square < mSize; square++) {
         for (direction = 0; direction < direction_count; direction++) {
            nextsquare = (mpBoard->*mDirections[direction])(color, square);
            while ( nextsquare != Board::OFF_BOARD ) {
               mAttackBitboards[color][direction][square].set(nextsquare);
               nextsquare = (mpBoard->*mDirections[direction])(color, nextsquare);
            }
         }
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  StaticDynamicPatternPiece
//      Method:  ~StaticDynamicPatternPiece
// Description:  Destroys this instance of a rook
//--------------------------------------------------------------------------
StaticDynamicPatternPiece::~StaticDynamicPatternPiece()
{
   int color;
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   for (color = 0; color < 2; color++) {
      for (direction = 0; direction < direction_count; direction++) {
         delete [] mAttackBitboards[color][direction];
      }
   }

   for (color = 0; color < 2; color++) {
      delete [] mStaticAttackBitboards[color];
   }
}

//--------------------------------------------------------------------------
//       Class:  StaticDynamicPatternPiece
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool StaticDynamicPatternPiece::ThreatensSquare(Piece* piece, int square)
{
   bool finished = false;
   int firstblocker;
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   Bitboard blockers(mSize), bitmoves(mSize);

   int start = piece->mSquare;
   int color = piece->mColor;

   // Test whether our target lies in the reach of our static attack patterns
   if ( mStaticAttackBitboards[color][start].test(square) )
      return true;

   // Attempt to find a direction in which we can move 
   // that will bring us to our target square
   direction = 0;
   while (!finished) {
      finished = mAttackBitboards[color][direction][start].test(square) ||
                 (++direction == direction_count);
   }
   
   // No direction found so we cannot threaten our target
   if (direction == direction_count) return false;

   // We can threaten our target, but we need to verify
   // that no pieces lie in between us and our target
   blockers = mAttackBitboards[color][direction][start] &
              mpBoard->mAllPieces;
   firstblocker = start;
   do {
      firstblocker = (mpBoard->*mDirections[direction])(color, firstblocker);
   } while ( (firstblocker != Board::OFF_BOARD) && 
             (!blockers.test(firstblocker) ) );

   return (firstblocker == square);
}

//--------------------------------------------------------------------------
//       Class:  StaticDynamicPatternPiece
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void StaticDynamicPatternPiece::GenerateCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int finish, firstblocker;
   Piece* captured;
   Bitboard blockers(mSize), bitmoves(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
   int color = piece->mColor;

   // First generate capture destinations via static move patterns
   bitmoves = mStaticAttackBitboards[color][start] &
              mpBoard->mColoredPieces[1-color];
   
   for (direction = 0; direction < direction_count; direction++) {
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[color][direction][start] &
                 mpBoard->mAllPieces;
   
      // Locate the first piece along this direction
      firstblocker = (mpBoard->*mDirections[direction])(0, start);
      while ( (firstblocker != Board::OFF_BOARD) && 
              (!blockers.test(firstblocker) ) ) 
      {
         firstblocker = (mpBoard->*mDirections[direction])(0, firstblocker);
      }
   
      // If a first blocking piece exists and it is not a friendly
      // piece, then we can capture it
      if ((firstblocker != Board::OFF_BOARD) &&
          mpBoard->mColoredPieces[1-color].test(firstblocker)) {
         bitmoves.set(firstblocker);
      }
   }

   // Create move list from our bitboard of possible destinations
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         captured = mpBoard->mSquares[finish];
         movelist.push_back( Move(piece, start, finish, captured) );
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  StaticDynamicPatternPiece
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void StaticDynamicPatternPiece::GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int finish, firstblocker, last;
   Bitboard blockers(mSize), bitmoves(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
   int color = piece->mColor;
 
   // First generate destinations via static move patterns
   bitmoves = mStaticAttackBitboards[color][start] &
              mpBoard->mNoPieces;
  
   for (direction = 0; direction < direction_count; direction++) {
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[color][direction][start] &
                 mpBoard->mAllPieces;
      
      // Find the first square along our direction _before_ we
      // encounter a blocking piece
      last = start;
      firstblocker = (mpBoard->*mDirections[direction])(0, start);
      while ( (firstblocker != Board::OFF_BOARD) && 
              (!blockers.test(firstblocker) ) ) 
      {
         last = firstblocker;
         firstblocker = (mpBoard->*mDirections[direction])(0, last);
      }
      
      // Make sure that we weren't immediately blocked
      if (last != start) {
         if (firstblocker == Board::OFF_BOARD) {
            // There was no blocking piece; add all the squares
            // along our chosen direction as possible destination squares
            bitmoves |= mAttackBitboards[color][direction][start];
         }
         else {
            // Add only those squares that lie on our path before
            // the first blocking square
            bitmoves |= mAttackBitboards[color][direction][start] ^
                        mAttackBitboards[color][direction][last];
         }
      }
   }

   // Create move list from our bitboard of possible destinations
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         movelist.push_back( Move(piece, start, finish) );
      }
   }
}
