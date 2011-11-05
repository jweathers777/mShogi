////////////////////////////////////////////////////////////////////////////
// Name:         FireDemon.cpp
// Description:  Implementation for a class that represents a fire demon
// Created:      09/02/2004 11:24:57 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "FireDemon.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  FireDemon
//      Method:  FireDemon
// Description:  Constructs an instance of a fire demon
//--------------------------------------------------------------------------
FireDemon::FireDemon(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "FiD";
   mNames[0] = "Fire Demon"; mNames[1] = "Kaki";
   mDescription =
      "Moves like a flying ox or a 3-step area mover and captures any adjacent enemies at the end of each move";

   // Grab a pointer to the area mover instance
   mpAreaMover = AreaMover::GetInstance(mpBoard);

   // Grab pointers to the area mover's attack bitboards and area paths
   mAreaAttackBitboards = mpAreaMover->mAttackBitboards;
   
   // Set the size of the directions vector
   mDirections.resize(DIRECTION_COUNT);
   
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTH] = &Board::North;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTH] = &Board::South;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   // Initialize the standard attack patterns
   InitStandardAttackPatterns();
   
   // Initialize burn patterns
   InitBurnPatterns();
}

//------------------------------------------------------------------------
//       Class:  FireDemon
//      Method:  InitStandardAttackPatterns
// Description:  Initial standard attack pattern bitboards
//------------------------------------------------------------------------
void FireDemon::InitStandardAttackPatterns()
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
//       Class:  FireDemon
//      Method:  InitBurnPatterns
// Description:  Initialize fire demon burn pattern bitboards
//------------------------------------------------------------------------
void FireDemon::InitBurnPatterns()
{
   int finish;
   
   // Create new bitboards for every square on the board
   mBurnBitboards = new Bitboard [mSize];

   // For every possible start square,
   // set bits for every square that is adjacent to it
   for (int start = 0; start < mSize; start++) {
      mBurnBitboards[start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
      // west
      finish = mpBoard->West(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
      // east
      finish = mpBoard->East(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
      // southwest
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
      // southeast
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mBurnBitboards[start].set(finish);
      }
   }  

}

//--------------------------------------------------------------------------
//       Class:  FireDemon
//      Method:  ~FireDemon
// Description:  Destroys this instance of a fire demon
//--------------------------------------------------------------------------
FireDemon::~FireDemon()
{
   for (int direction = 0; direction < DIRECTION_COUNT; direction++) {
      delete [] mAttackBitboards[direction];
   }
   
   delete [] mBurnBitboards;
}

//--------------------------------------------------------------------------
//       Class:  FireDemon
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool FireDemon::ThreatensSquare(Piece* piece, int square)
{
   bool finished;
   int firstblocker, direction;
   int start = piece->mSquare;
   int color = piece->mColor;
   Bitboard adjacent(mSize), blockers(mSize), bitmoves(mSize);

   // Generate the adjacent squares, as we could threaten this
   // piece by threatening any of them as well
   adjacent = mBurnBitboards[square] & 
              ~mpBoard->mColoredPieces[color];
   adjacent.set(start);
   
   for (start = 0; start < mSize; start++) {
      if (adjacent.test(start)) {
         // First check if we can threaten our target with an area move
         if ( mAreaAttackBitboards[start].test(square) &&
              mpAreaMover->HasLegalPath(start, square) )
            return true;
             
         // Attempt to find a direction in which we can move 
         // that will bring us to our target square
         direction = 0;
         finished = false;
         while (!finished) {
            finished = mAttackBitboards[direction][start].test(square) ||
                       (++direction == DIRECTION_COUNT);
         }
         
         // No direction found so we cannot threaten our target
         if (direction == DIRECTION_COUNT) continue;

         // We can threaten our target, but we need to verify
         // that no pieces lie in between us and our target
         blockers = mAttackBitboards[direction][start] &
                    mpBoard->mAllPieces;

         firstblocker = start;
         do {
            firstblocker = (mpBoard->*mDirections[direction])(0,firstblocker);
         } while ( (firstblocker != Board::OFF_BOARD) &&
                   (!blockers.test(firstblocker) ) ); 

         if (firstblocker == square) return true;
      }
   }

   return false;
}

//--------------------------------------------------------------------------
//       Class:  FireDemon
//      Method:  GenerateCaptures
// Description:  Generate all moves for the given fire demon and 
//               append them to the given list. Note for Fire 
//               Demons this method generates all moves because 
//               it would be wasteful to generate non captures 
//               separately due to possible burn captures
//--------------------------------------------------------------------------
void FireDemon::GenerateCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int square, finish, firstblocker, direction;
   Piece* captured;
   vector<Piece*> captures;
   Bitboard blockers(mSize), areabitmoves(mSize), bitmoves(mSize);
   Bitboard totalbitmoves(mSize),burntpieces(mSize), zero(mSize);
   
   int start = piece->mSquare, last;
   int color = piece->mColor;
   int enemy = 1 - color;

   // First calculate any destination squares that 
   // are possible by making a pseudo-legal area move
   areabitmoves = mAreaAttackBitboards[start] &
                  ~mpBoard->mColoredPieces[color];
    
   // Now check for dynamic bit pattern moves
   for (direction = 0; direction < DIRECTION_COUNT; direction++) {
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
            // Add those squares that lie on our path before
            // the first blocking square
            bitmoves |= mAttackBitboards[direction][start] ^
                        mAttackBitboards[direction][last];
            // Add the first blocking square as a capture 
            // move if it is an enemy piece
            captured = mpBoard->mSquares[firstblocker];
            if (captured->mColor == enemy) {
               bitmoves.set(firstblocker);
            }
         }
      }
   }

   totalbitmoves = areabitmoves | bitmoves;

   // Create move list from our bitboard of possible destinations
   for (finish = 0; finish < mSize; finish++) {
      if ( totalbitmoves.test(finish) ) {
         captures.clear();
         // Test for a capture on the destination square
         captured = mpBoard->mSquares[finish];
         if (captured) {
            captures.push_back(captured);
         }
         
         // Test for burnt pieces
         burntpieces = mBurnBitboards[finish] &
                       mpBoard->mColoredPieces[enemy];
         if (burntpieces != zero) {
            for (square = 0; square < mSize; square++) {
               if (burntpieces.test(square)) {
                  captured = mpBoard->mSquares[square];
                  // Make sure that we have a capture here
                  // and that it is not another fire demon
                  if (captured->mTypeValue != mTypeValue) {
                     captures.push_back(captured);
                  }
               }
            }
         }
         
         movelist.push_back( 
            Move(piece, start, finish, captures, !bitmoves.test(finish)) );
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  FireDemon
//      Method:  GenerateNonCaptures
// Description:  Dummy method. All moves for fire demons are 
//               generated under GenerateCaptures because of 
//               the possibility of burn captures
//--------------------------------------------------------------------------
void FireDemon::GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece)
{
}
