////////////////////////////////////////////////////////////////////////////
// Name:         LionHawk.cpp
// Description:  Implementation for a class that represents a lion hawk
// Created:      09/01/2004 02:15:47 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "LionHawk.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  LionHawk
//      Method:  LionHawk
// Description:  Constructs an instance of a lion hawk
//--------------------------------------------------------------------------
LionHawk::LionHawk(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "LHk";
   mNames[0] = "Lion Hawk"; mNames[1] = "Shiyo";
   mDescription =
      "Moves like a lion or a bishop";

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
   InitLionAttackPatterns();
}

//------------------------------------------------------------------------
//       Class:  LionHawk
//      Method:  InitStandardAttackPatterns
// Description:  Initial standard attack pattern bitboards
//------------------------------------------------------------------------
void LionHawk::InitStandardAttackPatterns()
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
//       Class:  LionHawk
//      Method:  InitLionAttackPatterns
// Description:  Initialize lion attack pattern bitboards
//------------------------------------------------------------------------
void LionHawk::InitLionAttackPatterns()
{
   int start, finish, startX, startY, finishX, finishY;
   int count, direction;
   bool finished;
   int width = mpBoard->GetWidth();
   int height = mpBoard->GetHeight();

   enum { RIGHT, LEFT, UP, DOWN };

   mLionAttackBitboards[TYPE_A] = new Bitboard [mSize];
   mLionAttackBitboards[TYPE_B] = new Bitboard [mSize];
   for (start = 0; start < mSize; start++) {
      // Create and initialize type A lion hawk attack pattern bitboards
      mLionAttackBitboards[TYPE_A][start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
      // west
      finish = mpBoard->West(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
      // east
      finish = mpBoard->East(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
      // southwest
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
      // southeast
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mLionAttackBitboards[TYPE_A][start].set(finish);
      }
   
      // Create and initialize type B lion hawk attack pattern bitboards
      mLionAttackBitboards[TYPE_B][start].resize(mSize);
      
      startX = start % width;
      startY = start / width;
      
      direction = RIGHT;
      count = 0;
      finished = false;
      finishX = startX - 2;
      finishY = startY - 2;
      // Traverse the parameter of the 3x3 square centered at "start"
      while (!finished) {
         // The algorithm passes over "imaginary" squares that 
         // lie off the board when "start" lies near the edges, 
         // but we weed these invalid squares out here
         if ( (finishX > -1) && (finishY > -1) &&
              (finishX < width) && (finishY < height) ) {
            finish = finishX + width*finishY;
            mLionAttackBitboards[TYPE_B][start].set(finish);
         }

         switch (direction) {
         case RIGHT: // Continuing stepping right until we hit the corner
            finishX++;
            count++;
            if (count == 5) {
               count = 0;
               direction = DOWN;
               finishX--;
            }
            break;
         case DOWN: // Continue stepping down until we hit the corner
            finishY++;
            count++;
            if (count == 5) {
               count = 0;
               direction = LEFT;
               finishY--;
            }
            break;
         case LEFT: // Continue stepping left until we hit the corner
            finishX--;
            count++;
            if (count == 5) {
               count = 0;
               direction = UP;
               finishX++;
            }
            break;
         case UP: // Continue stepping up until we hit the corner
            finishY--;
            count++;
            if (count == 4) {
               finished = true;
            }
            break;
         }
      }
   }  
}

//--------------------------------------------------------------------------
//       Class:  LionHawk
//      Method:  ~LionHawk
// Description:  Destroys this instance of a static pattern piece
//--------------------------------------------------------------------------
LionHawk::~LionHawk()
{
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();
   for (direction = 0; direction < direction_count; direction++) {
      delete [] mAttackBitboards[direction];
   }

   for (int i = 0; i < 2; i++) {
      delete [] mLionAttackBitboards[i];
   }
}

//--------------------------------------------------------------------------
//       Class:  LionHawk
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool LionHawk::ThreatensSquare(Piece* piece, int square)
{
   int start = piece->mSquare;

   // Is the distance between us and our target square
   // greater than two (i.e. outside of our lion attack powers)?
   if (mpBoard->Distance(start, square) > 2) {
      // We're outside the reach of lion attacks and need only test
      // whether we can threaten with bishop attacks
      int firstblocker;
      vector<Bitboard*>::size_type direction;
      vector<Bitboard*>::size_type direction_count = mDirections.size();
      Bitboard blockers(mSize), bitmoves(mSize);

      // Attempt to find a direction in which we can move 
      // that will bring us to our target square
      direction = 0;
      bool finished = false;
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
   else {
      // We're within reach of the lion attack; we only need to test
      // whether we can threaten via lion attacks because bishop attacks
      // within radius 2 are a subset of lion attacks
      
      // Can we make a single step Type A move to threaten our target?
      if (mLionAttackBitboards[TYPE_A][start].test(square)) return true;
      
      // Can we make a single or two step Type B move to threaten our target?
      if (mLionAttackBitboards[TYPE_B][start].test(square)) return true;
   }
   
   return false;
}

//--------------------------------------------------------------------------
//       Class:  LionHawk
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void LionHawk::GenerateCaptures(std::vector<Move>& movelist, 
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

   // First calculate single step Type A captures
   bitmoves = mLionAttackBitboards[TYPE_A][start] &
              mpBoard->mColoredPieces[enemy];
   for (finish = 0; finish < mSize; finish++) {
       if ( bitmoves.test(finish) ) {
         // Construct a move from this single-step destination
         captured = mpBoard->mSquares[finish];
         movelist.push_back( Move(piece, start, finish, captured) );
         
         // Generate igui capture move
         movelist.push_back( Move(piece, start, finish, start, captured) );

         // Generate two step destination squares from this
         // single-step destination square
         bitmoves2 = mLionAttackBitboards[TYPE_A][finish] &
                     ~mpBoard->mColoredPieces[color];

         // Construct moves from the destination squares
         for (finish2 = 0; finish2 < mSize; finish2++) {
            if (bitmoves2.test(finish2)) {
               captured2 = mpBoard->mSquares[finish2];
               if (captured2) { // Two capture two step move
                  captures.clear();
                  captures.push_back(captured);
                  captures.push_back(captured2);
                  movelist.push_back(
                     Move(piece, start, finish, finish2, captures) );
               }
               else { // One capture two step move
                  movelist.push_back(
                     Move(piece, start, finish, finish2, captured) );
               }
            }
         }
      }
   }
     
   // Now calculate one step Type B captures
   bitmoves = mLionAttackBitboards[TYPE_B][start] &
              mpBoard->mColoredPieces[enemy];

   // Now calculate and add any bishop move capture squares
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
//       Class:  LionHawk
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void LionHawk::GenerateNonCaptures(std::vector<Move>& movelist, 
                                             Piece* piece)
{
   int finish, firstblocker, last;
   Bitboard blockers(mSize), bitmoves(mSize), zero(mSize);
   vector<Bitboard*>::size_type direction;
   vector<Bitboard*>::size_type direction_count = mDirections.size();

   int start = piece->mSquare;

   // Calculate Type A non-capture destinations
   bitmoves = mLionAttackBitboards[TYPE_A][start] &
              mpBoard->mNoPieces;
   
   // Test whether we have any empty adjacent squares
   if (bitmoves != zero) {
      // If we are not surrounded by pieces, then
      // we can use a vacant adjacent square to perform
      // an igui move that effectively passes our turn
      movelist.push_back( Move(piece, start, start) );
   }

   // Add Type B non-capture destinations
   bitmoves |= mLionAttackBitboards[TYPE_B][start] &
               mpBoard->mNoPieces;

   // Now calculate and add any bishop move non-capture squares
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
            bitmoves |= mAttackBitboards[direction][start] ^
                        mAttackBitboards[direction][last];
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
