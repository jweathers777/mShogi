////////////////////////////////////////////////////////////////////////////
// Name:         SoaringEagle.hpp
// Description:  Interface for class that represents a soaring eagle
// Created:      09/02/2004 11:02:00 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __SoaringEagle_HPP__
#define __SoaringEagle_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class SoaringEagle : public PieceType
{
public:
   // ==========================================================================
   // Type definitions
   // ==========================================================================
   typedef int (Board::*DirectionMethodPtr)(int, int);

   // ==========================================================================
   // Construction
   // ==========================================================================
   SoaringEagle(Board* board, int value, int typevalue);

   void InitStandardAttackPatterns();

   virtual ~SoaringEagle();
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   bool ThreatensSquare(Piece* piece, int square);
   void GenerateCaptures(std::vector<Move>& movelist, Piece* piece);
   void GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece);
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, WEST, EAST, SOUTHWEST, SOUTH, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 6 };

   // ==========================================================================
   // Data
   // ==========================================================================
   Board* mpBoard;                    // Pointer to the game's board
   int    mSize;                      // Board size

   // Vector of pointers to Board methods for move directions
   std::vector<DirectionMethodPtr> mDirections;

   // Vector of arrays of Bitboards for calculating standard moves
   std::vector<Bitboard*> mAttackBitboards[2];
};

#endif

