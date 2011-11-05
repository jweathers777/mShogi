////////////////////////////////////////////////////////////////////////////////
// Name:         Board.hpp
// Description:  Interface for a class that represents a game board
// Created:      Apr 20, 2004 5:37:20 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef BOARD_HPP
#define BOARD_HPP

// STL header files
#include <vector>

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "common.hpp"
#include "PieceInfo.hpp"

// Forward declarations
class Piece;
class Move;

class Board
{
public:
   // ==========================================================================
	// Constructors and Destructors
	// ==========================================================================
	Board(int width, int height, int zonerow, int piecetypes); 
	virtual ~Board();
   
   // ==========================================================================
	// Accessors
	// ==========================================================================
   int GetSize() { return mSize; }
   int GetWidth() { return mWidth; }
   int GetHeight() { return mHeight; }

   void GetPieceInfo(std::vector<PieceInfo>& squares);

   uint64 KeyComponents(int color, int type, int square)
   {
      return mpKeyComponents[color + 2*type + 2*mPieceTypes*square];
   }
   
   // ==========================================================================
	// Operations
	// ==========================================================================
   virtual uint64 MakeMove(Move* move);
   virtual uint64 UnmakeMove(Move* move);

   void ResyncBitboards();
   
   // Returns the distance between two squares on the board
   int Distance(int p0, int p1)
   {
      int x0 = p0 % mWidth;
      int y0 = p0 / mWidth;
      int x1 = p1 % mWidth;
      int y1 = p1 / mWidth;

      int d0 = abs(x0 - x1);
      int d1 = abs(y0 - y1);

      return ( (d0 >= d1) ? d0 : d1 );
   }
   
   // Returns the square north of the argument square with respect to color
   int North(int color, int square)
   {
      if (color == BLACK) {
         if (square >= mWidth)
            return square - mWidth;
      }
      else {
         if (square < mSize - mWidth)
            return square + mWidth;
      }

      return OFF_BOARD;
   }

   // Returns the square west of the argument square with respect to color
   int West(int color, int square)
   {
      if (color == BLACK) {
         if (square % mWidth != mWidth - 1)
            return square + 1;
      }
      else {
         if (square % mWidth != 0)
            return square - 1;
      }

      return OFF_BOARD;
   }
   
   // Returns the square east of the argument square with respect to color
   int East(int color, int square)
   {
      if (color == BLACK) {
         if (square % mWidth != 0)
            return square - 1;
      }
      else {
         if (square % mWidth != mWidth - 1)
            return square + 1;
      }

      return OFF_BOARD;
   }
   
   // Returns the square south of the argument square with respect to color
   int South(int color, int square)
   {
      if (color == BLACK) {
         if (square < mSize - mWidth)
            return square + mWidth;
      }
      else {
         if (square >= mWidth)
            return square - mWidth;
      }

      return OFF_BOARD;
   }
   
   // Returns the square north west of the argument square with respect to color
   int NorthWest(int color, int square)
   {
      if (color == BLACK) {
         if ( (square >= mWidth) &&
              (square % mWidth != mWidth - 1) )
            return square - mWidth + 1;
      }
      else {
         if ( (square < mSize - mWidth) &&
              (square % mWidth != 0) )
            return square + mWidth - 1;
      }

      return OFF_BOARD;
   }
   // Returns the square north east of the argument square with respect to color
   int NorthEast(int color, int square)
   {
      if (color == BLACK) {
         if ( (square >= mWidth) &&
              (square % mWidth != 0) )
            return square - mWidth - 1;
      }
      else {
         if ( (square < mSize - mWidth) &&
              (square % mWidth != mWidth - 1) )
            return square + mWidth + 1;
      }

      return OFF_BOARD;
   }
   // Returns the square south west of the argument square with respect to color
   int SouthWest(int color, int square)
   {
      if (color == BLACK) {
         if ( (square < mSize - mWidth) &&
              (square % mWidth != mWidth - 1) )
            return square + mWidth + 1;
      }
      else {
         if ( (square >= mWidth) &&
              (square % mWidth != 0) )
            return square - mWidth - 1;
      }

      return OFF_BOARD;
   }
   // Returns the square south east of the argument square with respect to color
   int SouthEast(int color, int square)
   {
      if (color == BLACK) {
         if ( (square < mSize - mWidth) &&
              (square % mWidth != 0) )
            return square + mWidth - 1;
      }
      else {
         if ( (square >= mWidth) &&
              (square % mWidth != mWidth - 1) )
            return square - mWidth + 1;
      }

      return OFF_BOARD;
   }

   // ==========================================================================
	// Constants
	// ==========================================================================
   static const int OFF_BOARD;
   
private:
   // ==========================================================================
	// Data
	// ==========================================================================
   int mHeight;   // Height of the board
   int mWidth;    // Width of the board
   int mSize;     // Size of the board
   
   int mPieceTypes; // Number of piece types

   // Components for 64-bit Zoberist hash signatures
   uint64* mpKeyComponents;

public:
   uint64 mHashKey;   // 64-bit Zoberist hash key
   
   std::vector<Piece*> mSquares; // Pointers to pieces on board squares
   Bitboard mColoredPieces[2];   // Bitboards of pieces of a given color
   Bitboard mAllPieces;          // Bitboard of squares with all pieces
   Bitboard mNoPieces;           // Bitboard of squares with no pieces
   Bitboard mPromotionZones[2];  // Bitboards that represent the 
                                 // promotion zone boundaries of both sides
};

#endif // BOARD_HPP
