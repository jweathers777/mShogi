////////////////////////////////////////////////////////////////////////////
// Name:         HornedFalcon.cpp
// Description:  Implementation for a class that represents a horned falcon
// Created:      09/02/2004 07:56:49 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "HornedFalcon.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  HornedFalcon
//      Method:  HornedFalcon
// Description:  Constructs an instance of a horned falcon
//--------------------------------------------------------------------------
HornedFalcon::HornedFalcon(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "HF";
   mNames[0] = "Horned Falcon"; mNames[1] = "Kakuo";
   mDescription =
      "Moves any number of squares in any direction except forward or like a Lion in the forward direction";

   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[WEST] = &Board::West;
   mDirections[EAST] = &Board::East;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTH] = &Board::South;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the standard attack patterns
   InitStandardAttackPatterns();
}

//------------------------------------------------------------------------
//       Class:  HornedFalcon
//      Method:  InitStandardAttackPatterns
// Description:  Initial standard attack pattern bitboards
//------------------------------------------------------------------------
void HornedFalcon::InitStandardAttackPatterns()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int color; 
   int square;
   int nextsquare;
   for (color = 0; color < 2; color++) {
      // Set the size of the attack boards vector
      mAttackBitboards[color].resize(direction_count);
      
      // Create new bitboard arrays for each direction
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
//       Class:  HornedFalcon
//      Method:  ~HornedFalcon
// Description:  Destroys this instance of a static pattern piece
//--------------------------------------------------------------------------
HornedFalcon::~HornedFalcon()
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
//       Class:  HornedFalcon
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool HornedFalcon::ThreatensSquare(Piece* piece, int square)
{
   int start = piece->mSquare;
   int color = piece->mColor;
   int finish;

   // Test for standard attack threats first
   int firstblocker;
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   Bitboard blockers(mSize), bitmoves(mSize);

   // Attempt to find a direction in which we can move 
   // that will bring us to our target square
   direction = 0;
   bool finished = false;
   while (!finished) {
      finished = mAttackBitboards[color][direction][start].test(square) ||
                 (++direction == direction_count);
   }
   
   // We found a direction in which we can attack via
   // the standard attack patterns
   if (direction != direction_count) {
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
   
   // Can we make a single step forward to threaten our target?
   finish = mpBoard->North(color, start);
   if (finish != Board::OFF_BOARD) {
      if (finish == square) return true;
      
      // Can we make a two step move forward to threaten our target?
      finish = mpBoard->North(color, finish);
      if (finish == square) return true;
   }
   
   return false;
}

//--------------------------------------------------------------------------
//       Class:  HornedFalcon
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void HornedFalcon::GenerateCaptures(std::vector<Move>& movelist, 
                                          Piece* piece)
{
   int firstblocker, finish, finish2;
   Piece *captured, *captured2;
   vector<Piece*> captures;
   Bitboard blockers(mSize),bitmoves(mSize), bitmoves2(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   
   int start = piece->mSquare;
   int color = piece->mColor;
   int enemy = 1 - piece->mColor;

   bool captureA = false;
   bool captureB = false;

   // First calculate lion attack captures in the forward direction
   finish = mpBoard->North(color, start);
   if (finish != Board::OFF_BOARD) {
      captured = mpBoard->mSquares[finish];
      captureA = captured && (captured->mColor == enemy);
      finish2 = mpBoard->North(color, finish);
      if (finish2 != Board::OFF_BOARD) {
         captured2 = mpBoard->mSquares[finish2];
         captureB = captured2 && (captured2->mColor == enemy);
      }
   }

   if (captureA) {
      // Construct a move from this single-step destination
      movelist.push_back( Move(piece, start, finish, captured) );
      
      // Generate igui capture move
      movelist.push_back( Move(piece, start, finish, start, captured) );

      if (captureB) {
         // Construct a double capture move
         captures.clear();
         captures.push_back(captured);
         captures.push_back(captured2);
         movelist.push_back( Move(piece, start, finish, finish2, captures) );
      }
   }

   if (captureB) {
      // Construct a move from this two-step destination
      movelist.push_back( Move(piece, start, finish2, captured2) );
   }
   
   // Now calculate any standard move capture squares
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
//       Class:  HornedFalcon
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void HornedFalcon::GenerateNonCaptures(std::vector<Move>& movelist, 
                                             Piece* piece)
{
   int finish, firstblocker, last;
   Bitboard blockers(mSize), bitmoves(mSize), zero(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();

   int start = piece->mSquare;
   int color = piece->mColor;

   // Test whether we can move forward
   finish = mpBoard->North(color, start);
   if (finish != Board::OFF_BOARD) {
      // Check whether or not there is a piece in front of us
      if (!mpBoard->mSquares[finish]) {
         // The square is empty so we can move there...
         movelist.push_back( Move(piece, start, finish) );
         // ... or use it to make an igui pass move
         movelist.push_back( Move(piece, start, start) );
      }

      // See whether we can move forward again
      finish = mpBoard->North(color, finish);
      if (finish != Board::OFF_BOARD) {
         // Check whether or not there is a piece on this square
         if (!mpBoard->mSquares[finish]) {
            // No square here, so we can add a move
            movelist.push_back( Move(piece, start, finish) );
         }
      }
   }

   // Now calculate and add any standard move non-capture squares
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
            bitmoves |= mAttackBitboards[color][direction][start] ^
                        mAttackBitboards[color][direction][last];
         }
      }
   }
   
   // Create moves for each non-capture destination square
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         movelist.push_back( Move(piece, start, finish) );
      }
   }
}
