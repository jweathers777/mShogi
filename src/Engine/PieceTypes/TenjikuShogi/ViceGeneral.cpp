////////////////////////////////////////////////////////////////////////////
// Name:         ViceGeneral.cpp
// Description:  Implementation for a class that represents a vice general
// Created:      08/29/2004 09:03:13 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "ViceGeneral.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;
using std::map;
using std::make_pair;

//--------------------------------------------------------------------------
//       Class:  ViceGeneral
//      Method:  ViceGeneral
// Description:  Constructs an instance of a vice general
//--------------------------------------------------------------------------
ViceGeneral::ViceGeneral(Board* board, int value, int typevalue, int rank,
                         map<int,int>* rankmap)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();
   mRank = rank;
      
   mpRankMap = rankmap;
   mpRankMap->insert(make_pair(typevalue, rank));

   mNotation = "VGn";
   mNames[0] = "Vice General"; mNames[1] = "Fukusho";
   mDescription =
      "Moves like a bishop general or a 3-step area mover";  

   // Grab a pointer to the area mover instance
   mpAreaMover = AreaMover::GetInstance(mpBoard);

   // Grab pointers to the area mover's attack bitboards and area paths
   mAreaAttackBitboards = mpAreaMover->mAttackBitboards;
   
   // Set up the directional method pointers
   mDirections[NORTHWEST] = &Board::NorthWest;
   mDirections[NORTHEAST] = &Board::NorthEast;
   mDirections[SOUTHWEST] = &Board::SouthWest;
   mDirections[SOUTHEAST] = &Board::SouthEast;

   int square, direction, next;
   
   // Initialize vice general's dynamic attack patterns
   for (direction = 0; direction < DIRECTIONS; direction++) {
      mAttackBitboards[direction] = new Bitboard [mSize];
   }
   
   for (direction = 0; direction < DIRECTIONS; direction++) {
      for (square = 0; square < mSize; square++) {
         mAttackBitboards[direction][square].resize(mSize);
         next = (mpBoard->*mDirections[direction])(0, square);
         while ( next != Board::OFF_BOARD ) {
            mAttackBitboards[direction][square].set(next);
            next = (mpBoard->*mDirections[direction])(0, next);
         }
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  ViceGeneral
//      Method:  ~ViceGeneral
// Description:  Destroys this instance of a vice general
//--------------------------------------------------------------------------
ViceGeneral::~ViceGeneral()
{
   for (int direction = 0; direction < DIRECTIONS; direction++) {
      delete [] mAttackBitboards[direction];
   }
}

//--------------------------------------------------------------------------
//       Class:  ViceGeneral
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool ViceGeneral::ThreatensSquare(Piece* piece, int square)
{
   int start = piece->mSquare;
   
   // First check if we can threaten our target with an area move
   if ( mAreaAttackBitboards[start].test(square) &&
        mpAreaMover->HasLegalPath(start, square) )
      return true;
       
   bool finished = false;
   int firstblocker, direction;
   Bitboard blockers(mSize), bitmoves(mSize);

   // Attempt to find a direction in which we can move 
   // that will bring us to our target square
   direction = 0;
   while (!finished) {
      finished = mAttackBitboards[direction][start].test(square) ||
                 (++direction == DIRECTIONS);
   }
   
   // No direction found so we cannot threaten our target
   if (direction == DIRECTIONS) return false;

   blockers = mAttackBitboards[direction][start] &
              mpBoard->mAllPieces;
   
   // We can threaten our target, but we need to verify
   // that no pieces lie in between us and our target that
   // have a rank equal to or greater than our own
   map<int,int>::iterator itr;
   firstblocker = start;
   while (true) {
      firstblocker = (mpBoard->*mDirections[direction])(0, firstblocker);
      if (firstblocker == square) return true; // We've reached our target
      if (firstblocker == Board::OFF_BOARD) return false; // We're off the board
      
      if (blockers.test(firstblocker)) {
         // Look up this potentially blocking piece's rank
         itr = mpRankMap->find(mpBoard->mSquares[firstblocker]->mTypeValue);
         if (itr != mpRankMap->end()) {
            // Is the blocking piece of higher or equal rank than this piece?
            if ((*itr).second >= mRank) {
               return false; // Can't jump higher or equal rank piece
            }
         }
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  ViceGeneral
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void ViceGeneral::GenerateCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int finish, firstblocker, direction;
   Piece* captured;
   Bitboard blockers(mSize), bitmoves(mSize);
   Bitboard areabitmoves(mSize), totalbitmoves(mSize);
   
   int start = piece->mSquare;
   int color = piece->mColor;

   // First calculate any destination squares that are possible
   // by making a pseudo-legal area move
   areabitmoves = mAreaAttackBitboards[start] &
                  mpBoard->mColoredPieces[1-color];
   
   map<int,int>::iterator itr;
   // Now check for dynamic bit pattern moves
   for (direction = 0; direction < DIRECTIONS; direction++) {
      // Find all pieces along our chosen direction from our starting point
      blockers = mAttackBitboards[direction][start] &
                 mpBoard->mAllPieces;
   
      // Locate the first piece along this direction
      firstblocker = (mpBoard->*mDirections[direction])(0, start);
      while (firstblocker != Board::OFF_BOARD) {
         if (blockers.test(firstblocker)) {
            // We've reached this piece, so we can capture it
            // if it isn't one of ours
            if (!mpBoard->mColoredPieces[color].test(firstblocker))
               bitmoves.set(firstblocker);
            
            // Look up this potentially blocking piece's rank
            itr = mpRankMap->find(mpBoard->mSquares[firstblocker]->mTypeValue);
            if (itr != mpRankMap->end()) {
               // Is the blocking piece of higher or equal rank than this piece?
               if ((*itr).second >= mRank) {
                  break; // Can't jump higher or equal rank piece
               }
            }
         }
         firstblocker = (mpBoard->*mDirections[direction])(0, firstblocker);
      }
   }

   totalbitmoves = areabitmoves | bitmoves;
   
   // Create move list from our bitboard of possible destinations
   for (finish = 0; finish < mSize; finish++) {
      if ( totalbitmoves.test(finish) ) {
         captured = mpBoard->mSquares[finish];
         movelist.push_back( 
            Move(piece, start, finish, captured, !bitmoves.test(finish)) );
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  ViceGeneral
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void ViceGeneral::GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece)
{
   int finish, firstblocker, direction, last;
   Bitboard blockers(mSize), bitmoves(mSize);
   Bitboard areabitmoves(mSize), totalbitmoves(mSize);
   
   int start = piece->mSquare;
   
   // First calculate any destination squares that are possible
   // by making a pseudo-legal area move
   areabitmoves = mAreaAttackBitboards[start] &
                  mpBoard->mNoPieces;

   // Now check for dynamic bit pattern moves
   for (direction = 0; direction < DIRECTIONS; direction++) {
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
   
   totalbitmoves = areabitmoves | bitmoves;

   // Create move list from our bitboard of possible destinations
   for (finish = 0; finish < mSize; finish++) {
      if ( totalbitmoves.test(finish) ) {
         movelist.push_back( Move(piece, start, finish, 0, !bitmoves.test(finish)) );
      }
   }
}
