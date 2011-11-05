////////////////////////////////////////////////////////////////////////////
// Name:         LimDynamicPatternPiece.cpp
// Description:  Implementation for class that represents a piece 
//               type whose move can be represented with a dynamic 
//               bit pattern with slide length limits
// Created:      09/01/2004 09:07:30 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:22:10 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "LimDynamicPatternPiece.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;

//------------------------------------------------------------------------
//       Class:  LimDynamicPatternPiece
//      Method:  InitAttackPatterns
// Description:  Initialize the attack pattern bitboards
//------------------------------------------------------------------------
void LimDynamicPatternPiece::InitAttackPatterns()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();

   // Set the size of each color's attack boards vector
   int color;
   for (color = 0; color < 2; color++) {
      mAttackBitboards[color].resize(direction_count);
   }
   
   int square, nextsquare, length;
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
            length = 1;
            while ( (nextsquare != Board::OFF_BOARD) &&
                    (length <= mSlideLimits[direction]) ) {
               mAttackBitboards[color][direction][square].set(nextsquare);
               nextsquare = (mpBoard->*mDirections[direction])(color, nextsquare);
               length++;
            }
         }
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  LimDynamicPatternPiece
//      Method:  ~LimDynamicPatternPiece
// Description:  Destroys this instance of a rook
//--------------------------------------------------------------------------
LimDynamicPatternPiece::~LimDynamicPatternPiece()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   for (int color = 0; color < 2; color++) {
      for (direction = 0; direction < direction_count; direction++) {
         delete [] mAttackBitboards[color][direction];
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  LimDynamicPatternPiece
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool LimDynamicPatternPiece::ThreatensSquare(Piece* piece, int square)
{
   bool finished = false;
   int firstblocker;
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   Bitboard blockers(mSize), bitmoves(mSize);

   int start = piece->mSquare;
   int color = piece->mColor;

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
//       Class:  LimDynamicPatternPiece
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void LimDynamicPatternPiece::GenerateCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int finish, firstblocker;
   Piece* captured;
   Bitboard blockers(mSize), bitmoves(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
   int color = piece->mColor;
   
   for (direction = 0; direction < direction_count; direction++) {
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[color][direction][start] &
                 mpBoard->mAllPieces;
   
      // Locate the first piece along this direction
      firstblocker = (mpBoard->*mDirections[direction])(color, start);
      while ( (firstblocker != Board::OFF_BOARD) && 
              (!blockers.test(firstblocker) ) ) 
      {
         firstblocker = (mpBoard->*mDirections[direction])(color, firstblocker);
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
//       Class:  LimDynamicPatternPiece
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void LimDynamicPatternPiece::GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int finish, firstblocker, last;
   Bitboard blockers(mSize), bitmoves(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
   int color = piece->mColor;
   
   for (direction = 0; direction < direction_count; direction++) {
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[color][direction][start] &
                 mpBoard->mAllPieces;
      
      // Find the first square along our direction _before_ we
      // encounter a blocking piece
      last = start;
      firstblocker = (mpBoard->*mDirections[direction])(color, start);
      while ( (firstblocker != Board::OFF_BOARD) && 
              (!blockers.test(firstblocker) ) ) 
      {
         last = firstblocker;
         firstblocker = (mpBoard->*mDirections[direction])(color, last);
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
            bitmoves |= mAttackBitboards[color][direction][start] &
                        ~mAttackBitboards[color][direction][last];
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
