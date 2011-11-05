////////////////////////////////////////////////////////////////////////////
// Name:         LionHawk.hpp
// Description:  Interface for class that represents a lion hawk
// Created:      09/01/2004 02:15:46 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __LionHawk_HPP__
#define __LionHawk_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class LionHawk : public PieceType
{
public:
   // ==========================================================================
   // Type definitions
   // ==========================================================================
   typedef int (Board::*DirectionMethodPtr)(int, int);

   // ==========================================================================
   // Construction
   // ==========================================================================
   LionHawk(Board* board, int value, int typevalue);

   void InitStandardAttackPatterns();
   void InitLionAttackPatterns();

   virtual ~LionHawk();
   
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
   enum { TYPE_A = 0, TYPE_B };
   
   enum {
      NORTHWEST = 0, NORTHEAST, SOUTHWEST, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 4 };

   // ==========================================================================
   // Data
   // ==========================================================================
   Board* mpBoard;                    // Pointer to the game's board
   int    mSize;                      // Board size
   Bitboard* mLionAttackBitboards[2]; // Array of bitboards for calculating moves

   // Vector of pointers to Board methods for move directions
   std::vector<DirectionMethodPtr> mDirections;

   // Vector of arrays of Bitboards for calculating standard moves
   std::vector<Bitboard*> mAttackBitboards;
};

#endif

