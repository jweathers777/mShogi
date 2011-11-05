////////////////////////////////////////////////////////////////////////////
// Name:         HeavenlyTetrarchs.cpp
// Description:  Implementation for a class that represents a heavenly tetrarchs
// Created:      09/04/2004 11:15:38 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "HeavenlyTetrarchs.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  HeavenlyTetrarchs
//      Method:  HeavenlyTetrarchs
// Description:  Constructs an instance of a heavenly tetrarchs
//--------------------------------------------------------------------------
HeavenlyTetrarchs::HeavenlyTetrarchs(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mWidth =mpBoard->GetWidth();
   mSize = mpBoard->GetSize();

   mNotation = "HT";
   mNames[0] = "Heavenly Tetrarchs"; mNames[1] = "Shitenno";
   mDescription =
      "Moves like a bishop but cannot land on any adjacent square, may igui capture, leap to second square horizontally, or take exactly 3 steps horizontally";

   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the standard attack patterns
   InitStandardAttackPatterns();
   
   // Initialize lion attack patterns
   InitIguiCapturePatterns();
}

//------------------------------------------------------------------------
//       Class:  HeavenlyTetrarchs
//      Method:  InitStandardAttackPatterns
// Description:  Initial standard attack pattern bitboards
//------------------------------------------------------------------------
void HeavenlyTetrarchs::InitStandardAttackPatterns()
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
   int nextsquare;
   for (square = 0; square < mSize; square++) {
      for (direction = 0; direction < direction_count; direction++) {
         nextsquare = (mpBoard->*mDirections[direction])(0, square);
         while ( nextsquare != Board::OFF_BOARD ) {
            mAttackBitboards[direction][square].set(nextsquare);
            nextsquare = (mpBoard->*mDirections[direction])(0, nextsquare);
         }
      }
   }
}

//------------------------------------------------------------------------
//       Class:  HeavenlyTetrarchs
//      Method:  InitIguiCapturePatterns
// Description:  Initialize igui capture pattern bitboards
//------------------------------------------------------------------------
void HeavenlyTetrarchs::InitIguiCapturePatterns()
{
   int finish;
   // Initialize the king's attack patterns
   mIguiCaptureBitboards = new Bitboard [mSize];
   for (int start = 0; start < mSize; start++) {
      mIguiCaptureBitboards[start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
      // west
      finish = mpBoard->West(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
      // east
      finish = mpBoard->East(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
      // southwest
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
      // southeast
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mIguiCaptureBitboards[start].set(finish);
      }
   }  
}

//--------------------------------------------------------------------------
//       Class:  HeavenlyTetrarchs
//      Method:  ~HeavenlyTetrarchs
// Description:  Destroys this instance of a static pattern piece
//--------------------------------------------------------------------------
HeavenlyTetrarchs::~HeavenlyTetrarchs()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   for (direction = 0; direction < direction_count; direction++) {
      delete [] mAttackBitboards[direction];
   }

   delete [] mIguiCaptureBitboards;
}

//--------------------------------------------------------------------------
//       Class:  HeavenlyTetrarchs
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool HeavenlyTetrarchs::ThreatensSquare(Piece* piece, int square)
{
   bool finished = false;
   int firstblocker;
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   Bitboard blockers(mSize), bitmoves(mSize);

   int start = piece->mSquare;
   int enemy = 1 - piece->mColor;

   // Test whether our target lies in the reach of our igui capture patterns
   if ( mIguiCaptureBitboards[start].test(square) &&
        mpBoard->mColoredPieces[enemy].test(square))
      return true;

   // Is our target square in reach of our horizontal jumps?
   int dif = (square % mWidth) - (start % mWidth);
   if (abs(dif) == 2) return true;

   // Is our target square in reach of our horizontal 3-step?
   if ((dif == 3) && 
       mpBoard->mNoPieces[start+1] &&
       mpBoard->mNoPieces[start+2])
      return true;

   if ((dif == -3) && 
       mpBoard->mNoPieces[start-1] &&
       mpBoard->mNoPieces[start-2])
      return true;   

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
   firstblocker = (mpBoard->*mDirections[direction])(0, start);
   do {
      firstblocker = (mpBoard->*mDirections[direction])(0, firstblocker);
   } while ( (firstblocker != Board::OFF_BOARD) &&
             (!blockers.test(firstblocker) ) ); 

   return (firstblocker == square);

}

//--------------------------------------------------------------------------
//       Class:  HeavenlyTetrarchs
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void HeavenlyTetrarchs::GenerateCaptures(std::vector<Move>& movelist, 
                                          Piece* piece)
{
   int finish, firstblocker;
   Piece* captured;
   Bitboard blockers(mSize), bitmoves(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
   int color = piece->mColor;
   int enemy = 1 - color;

   // First generate capture destinations via igui capture patterns
   bitmoves = mIguiCaptureBitboards[start] &
              mpBoard->mColoredPieces[1-color];

   // Create move list from our igui moves
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         captured = mpBoard->mSquares[finish];
         movelist.push_back( Move(piece, start, finish, start, captured) );
         bitmoves.reset(finish);
      }
   }

   // Generate possible jump captures and step captures
   bool stop = false;
   finish = mpBoard->West(0, start);
   if (finish != Board::OFF_BOARD) {
      if (mpBoard->mNoPieces.test(finish)) {
         finish = mpBoard->West(0, finish);
         if (finish != Board::OFF_BOARD) {
            if (mpBoard->mNoPieces.test(finish)) {
               stop = true;
               finish = mpBoard->West(0, finish);
               if (finish != Board::OFF_BOARD) {
                  if (mpBoard->mColoredPieces[enemy].test(finish)) {
                     captured = mpBoard->mSquares[finish];
                     movelist.push_back( 
                        Move(piece, start, finish, captured) );
                  }
               }
            }
         }
      }
      if (!stop) {
         finish = mpBoard->West(0, finish);
         if (finish != Board::OFF_BOARD) {
            if (mpBoard->mColoredPieces[enemy].test(finish)) {
               captured = mpBoard->mSquares[finish];
               movelist.push_back( 
                 Move(piece, start, finish, captured) );
            }
         }
      }
   }                    
 
   stop = false;
   finish = mpBoard->East(0, start);
   if (finish != Board::OFF_BOARD) {
      if (mpBoard->mNoPieces.test(finish)) {
         finish = mpBoard->East(0, finish);
         if (finish != Board::OFF_BOARD) {
            if (mpBoard->mNoPieces.test(finish)) {
               stop = true;
               finish = mpBoard->East(0, finish);
               if (finish != Board::OFF_BOARD) {
                  if (mpBoard->mColoredPieces[enemy].test(finish)) {
                     captured = mpBoard->mSquares[finish];
                     movelist.push_back( 
                        Move(piece, start, finish, captured) );
                  }
               }
            }
         }
      }
      if (!stop) {
         finish = mpBoard->East(0, finish);
         if (finish != Board::OFF_BOARD) {
            if (mpBoard->mColoredPieces[enemy].test(finish)) {
               captured = mpBoard->mSquares[finish];
               movelist.push_back( 
                 Move(piece, start, finish, captured) );
            }
         }
      }
   }                    
 
   // Now calculate any bishop like captures 
   for (direction = 0; direction < direction_count; direction++) {
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[direction][start] &
                 mpBoard->mAllPieces;
   
      // Locate the first piece along this direction
      firstblocker = (mpBoard->*mDirections[direction])(0, start);
      if (firstblocker == Board::OFF_BOARD) continue;
      firstblocker = (mpBoard->*mDirections[direction])(0, firstblocker);
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
//       Class:  HeavenlyTetrarchs
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void HeavenlyTetrarchs::GenerateNonCaptures(std::vector<Move>& movelist, 
                                             Piece* piece)
{
   int finish, firstblocker, last;
   Bitboard blockers(mSize), bitmoves(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
 
   // Generate possible jump non-captures and step non-captures
   finish = mpBoard->West(0, start);
   if (finish != Board::OFF_BOARD) {
      if (mpBoard->mNoPieces.test(finish)) {
         finish = mpBoard->West(0, finish);
         if (finish != Board::OFF_BOARD) {
            if (mpBoard->mNoPieces.test(finish)) {
               finish = mpBoard->West(0, finish);
               if (finish != Board::OFF_BOARD) {
                  if (mpBoard->mNoPieces.test(finish)) {
                     movelist.push_back( Move(piece, start, finish) );
                  }
               }
            }
         }
      }
      finish = mpBoard->West(0, finish);
      if (finish != Board::OFF_BOARD) {
         if (mpBoard->mNoPieces.test(finish)) {
            movelist.push_back( Move(piece, start, finish) );
         }
      }
   }                    
 
   finish = mpBoard->East(0, start);
   if (finish != Board::OFF_BOARD) {
      if (mpBoard->mNoPieces.test(finish)) {
         finish = mpBoard->East(0, finish);
         if (finish != Board::OFF_BOARD) {
            if (mpBoard->mNoPieces.test(finish)) {
               finish = mpBoard->East(0, finish);
               if (finish != Board::OFF_BOARD) {
                  if (mpBoard->mNoPieces.test(finish)) {
                     movelist.push_back( Move(piece, start, finish) );
                  }
               }
            }
         }
      }
      finish = mpBoard->East(0, finish);
      if (finish != Board::OFF_BOARD) {
         if (mpBoard->mNoPieces.test(finish)) {
            movelist.push_back( Move(piece, start, finish) );
         }
      }
   }                    
  
   int nextsquare;
   // Now calculate any bishop like non-captures excluding
   // the adjacent squares 
   for (direction = 0; direction < direction_count; direction++) {
      nextsquare = (mpBoard->*mDirections[direction])(0, start);
      if (nextsquare == Board::OFF_BOARD) continue;
      
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[direction][nextsquare] &
                 mpBoard->mAllPieces;
      
      // Find the first square along our direction _before_ we
      // encounter a blocking piece
      last = nextsquare;
      firstblocker = (mpBoard->*mDirections[direction])(0, nextsquare);
      while ( (firstblocker != Board::OFF_BOARD) && 
              (!blockers.test(firstblocker) ) ) 
      {
         last = firstblocker;
         firstblocker = (mpBoard->*mDirections[direction])(0, last);
      }
      
      // Make sure that we weren't immediately blocked
      if (last != nextsquare) {
         if (firstblocker == Board::OFF_BOARD) {
            // There was no blocking piece; add all the squares
            // along our chosen direction as possible destination squares
            bitmoves |= mAttackBitboards[direction][nextsquare];
         }
         else {
            // Add only those squares that lie on our path before
            // the first blocking square
            bitmoves |= mAttackBitboards[direction][nextsquare] ^
                        mAttackBitboards[direction][last];
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
