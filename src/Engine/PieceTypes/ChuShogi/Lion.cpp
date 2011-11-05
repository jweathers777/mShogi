////////////////////////////////////////////////////////////////////////////
// Name:         Lion.cpp
// Description:  Implementation for a class that represents a lion
// Created:      09/01/2004 09:23:57 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Lion.hpp"
#include "Piece.hpp"
#include "Move.hpp"
#include "Board.hpp"

using std::vector;
using std::string;

// ==========================================================================
// Class Variable Initialization
// ==========================================================================
bool Lion::sLionCaptureRules = false;

//--------------------------------------------------------------------------
//       Class:  Lion
//      Method:  Lion
// Description:  Constructs an instance of a lion
//--------------------------------------------------------------------------
Lion::Lion(Board* board, int value, int typevalue)
{
   mpBoard = board; 
   mValue = value;
   mTypeValue = typevalue;
   mSize = mpBoard->GetSize();

   mNotation = "Ln";
   mNames[0] = "Lion"; mNames[1] = "Shishi";
   mDescription =
      "Takes up to two steps leaping or capturing as it goes and possibly returning to its starting square";

   int start, finish, startX, startY, finishX, finishY;
   int count, direction;
   bool finished;
   int width = mpBoard->GetWidth();
   int height = mpBoard->GetHeight();

   enum { RIGHT, LEFT, UP, DOWN };

   mAttackBitboards[TYPE_A] = new Bitboard [mSize];
   mAttackBitboards[TYPE_B] = new Bitboard [mSize];
   for (start = 0; start < mSize; start++) {
      // Create and initialize type A lion attack pattern bitboards
      mAttackBitboards[TYPE_A][start].resize(mSize);
      
      // northwest 
      finish = mpBoard->NorthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
      // north
      finish = mpBoard->North(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
      // northeast
      finish = mpBoard->NorthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
      // west
      finish = mpBoard->West(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
      // east
      finish = mpBoard->East(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
      // southwest
      finish = mpBoard->SouthWest(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
      // south
      finish = mpBoard->South(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
      // southeast
      finish = mpBoard->SouthEast(0, start);
      if (finish != Board::OFF_BOARD) {
         mAttackBitboards[TYPE_A][start].set(finish);
      }
   
      // Create and initialize type B lion attack pattern bitboards
      mAttackBitboards[TYPE_B][start].resize(mSize);
      
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
            mAttackBitboards[TYPE_B][start].set(finish);
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
//       Class:  Lion
//      Method:  ~Lion
// Description:  Destroys this instance of a static pattern piece
//--------------------------------------------------------------------------
Lion::~Lion()
{
   for (int i = 0; i < 2; i++) {
      delete [] mAttackBitboards[i];
   }
}

//--------------------------------------------------------------------------
//       Class:  Lion
//      Method:  ThreatensSquare
// Description:  Returns true if a given piece threatens the given square
//--------------------------------------------------------------------------
bool Lion::ThreatensSquare(Piece* piece, int square)
{
   bool result = false;
   
   // Can we make a single step Type A move to threaten our target?
   if (mAttackBitboards[TYPE_A][piece->mSquare].test(square)) {
      result = true;
   }
   // Can we make a single or two step Type B move to threaten our target?
   else if (mAttackBitboards[TYPE_B][piece->mSquare].test(square)) {
      result = true;
   }
   
   return result;
}

//--------------------------------------------------------------------------
//       Class:  Lion
//      Method:  GenerateCaptures
// Description:  Generates capture moves
//--------------------------------------------------------------------------
void Lion::GenerateCaptures(std::vector<Move>& movelist, 
                                          Piece* piece)
{
   bool validate;
   int finish, finish2;
   Piece *captured, *captured2;
   vector<Piece*> captures;
   Bitboard bitmoves(mSize), bitmoves2(mSize);
   
   int start = piece->mSquare;
   int color = piece->mColor;
   int enemy = 1 - piece->mColor;

   // First calculate single step Type A captures
   bitmoves = mAttackBitboards[TYPE_A][start] &
              mpBoard->mColoredPieces[enemy];
   for (finish = 0; finish < mSize; finish++) {
       if ( bitmoves.test(finish) ) {
         // Construct a move from this single-step destination
         captured = mpBoard->mSquares[finish];
         assert(captured != 0);
         movelist.push_back( Move(piece, start, finish, captured) );
         
         // Generate igui capture move
         movelist.push_back( Move(piece, start, finish, start, captured) );

         // Generate two step destination squares from this
         // single-step destination square
         bitmoves2 = mAttackBitboards[TYPE_A][finish] &
                     ~mpBoard->mColoredPieces[color];

         // Construct moves from the destination squares
         for (finish2 = 0; finish2 < mSize; finish2++) {
            if (bitmoves2.test(finish2)) {
               captured2 = mpBoard->mSquares[finish2];
               if (captured2) { // Two capture two step move
                  captures.clear();
                  captures.push_back(captured);
                  captures.push_back(captured2);
                  validate = sLionCaptureRules && 
                            (captured2->mTypeValue == mTypeValue);
                  movelist.push_back(
                     Move(piece, start, finish, finish2, captures, validate) );
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
   bitmoves = mAttackBitboards[TYPE_B][start] &
              mpBoard->mColoredPieces[enemy];
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         captured = mpBoard->mSquares[finish];
         validate = sLionCaptureRules && 
                    (captured->mTypeValue == mTypeValue);
         movelist.push_back( Move(piece, start, finish, captured, validate) );
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  Lion
//      Method:  GenerateNonCaptures
// Description:  Generates noncapture moves
//--------------------------------------------------------------------------
void Lion::GenerateNonCaptures(std::vector<Move>& movelist, 
                                             Piece* piece)
{
   int finish;
   Bitboard bitmoves(mSize), zero(mSize);

   int start = piece->mSquare;

   // Calculate Type A non-capture destinations
   bitmoves = mAttackBitboards[TYPE_A][start] &
              mpBoard->mNoPieces;
   
   // Test whether we have any empty adjacent squares
   if (bitmoves != zero) {
      // If we are not surrounded by pieces, then
      // we can use a vacant adjacent square to perform
      // an igui move that effectively passes our turn
      movelist.push_back( Move(piece, start, start) );
   }

   // Add Type B non-capture destinations
   bitmoves |= mAttackBitboards[TYPE_B][start] &
               mpBoard->mNoPieces;

   // Create moves for each non-capture destination square
   for (finish = 0; finish < mSize; finish++) {
      if ( bitmoves.test(finish) ) {
         movelist.push_back( Move(piece, start, finish) );
      }
   }
}
