////////////////////////////////////////////////////////////////////////////
// Name:         LimSymDynPatternPiece.cpp
// Description:  Interface for class that represents a piece type 
//               whose move can be represented with a symmetric 
//               dynamic bit pattern with slide length limits
// Created:      08/31/2004 05:04:01 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:22:10 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "LimSymDynPatternPiece.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;

//------------------------------------------------------------------------
//       Class:  LimSymDynPatternPiece
//      Method:  InitAttackPatterns
// Description:  Initialize the attack pattern bitboards
//------------------------------------------------------------------------
void LimSymDynPatternPiece::InitAttackPatterns()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   // Set the size of the attack boards vector
   mAttackBitboards.resize(direction_count);
   
   // Create new bitboard arrays for each direction
   for (direction = 0; direction < direction_count; direction++) {
      mAttackBitboards[direction] = new Bitboard [mSize];
   }
   
   // Resize each bitboard to fit the current board size
   int square;
   for (square = 0; square < mSize; square++) {
      for (direction = 0; direction < direction_count; direction++) {
         mAttackBitboards[direction][square].resize(mSize);
      }
   }  

   // Fill each bitboard with the appropriate bit 
   // pattern based upon the direction of movement
   int length, nextsquare;
   for (square = 0; square < mSize; square++) {
      for (direction = 0; direction < direction_count; direction++) {
         nextsquare = (mpBoard->*mDirections[direction])(0, square);
         length = 1;
         while ( (nextsquare != Board::OFF_BOARD) &&
                 (length <= mSlideLimits[direction]) ) {
            mAttackBitboards[direction][square].set(nextsquare);
            nextsquare = (mpBoard->*mDirections[direction])(0, nextsquare);
            length++;
         }
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  LimSymDynPatternPiece
//      Method:  ~LimSymDynPatternPiece
// Description:  Destroys this instance of a rook
//--------------------------------------------------------------------------
LimSymDynPatternPiece::~LimSymDynPatternPiece()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   for (direction = 0; direction < direction_count; direction++) {
      delete [] mAttackBitboards[direction];
   }
}

//--------------------------------------------------------------------------
//       Class:  LimSymDynPatternPiece
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool LimSymDynPatternPiece::ThreatensSquare(Piece* piece, int square)
{
   bool finished = false;
   int firstblocker;
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   Bitboard blockers(mSize), bitmoves(mSize);

   int start = piece->mSquare;

   // Attempt to find a direction in which we can move 
   // that will bring us to our target square
   direction = 0;
   while (!finished) {
      finished = mAttackBitboards[direction][start].test(square) ||
                 (++direction == direction_count);
   }
   
   // No direction found so we cannot threaten our target
   if (direction == direction_count) return false;

   // We can threaten our target, but we need to verify
   // that no pieces lie in between us and our target
   blockers = mAttackBitboards[direction][start] &
              mpBoard->mAllPieces;
   firstblocker = start;
   do {
      firstblocker = (mpBoard->*mDirections[direction])(0, firstblocker);
   } while ( (firstblocker != Board::OFF_BOARD) &&
             (!blockers.test(firstblocker) ) ); 

   return (firstblocker == square);
}

//--------------------------------------------------------------------------
//       Class:  LimSymDynPatternPiece
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void LimSymDynPatternPiece::GenerateCaptures(std::vector<Move>& movelist, Piece* piece)
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
      blockers = mAttackBitboards[direction][start] &
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
//       Class:  LimSymDynPatternPiece
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void LimSymDynPatternPiece::GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int finish, firstblocker, last;
   Bitboard blockers(mSize), bitmoves(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
   
   for (direction = 0; direction < direction_count; direction++) {
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[direction][start] &
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
            bitmoves |= mAttackBitboards[direction][start];
         }
         else {
            // Add only those squares that lie on our path before
            // the first blocking square
            bitmoves |= mAttackBitboards[direction][start] &
                        ~mAttackBitboards[direction][last];
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
