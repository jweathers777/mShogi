////////////////////////////////////////////////////////////////////////////
// Name:         HeavenlyTetrarchs.hpp
// Description:  Interface for class that represents a heavenly tetrarchs
// Created:      09/04/2004 11:15:37 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __HeavenlyTetrarchs_HPP__
#define __HeavenlyTetrarchs_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class HeavenlyTetrarchs : public PieceType
{
public:
   // ==========================================================================
   // Type definitions
   // ==========================================================================
   typedef int (Board::*DirectionMethodPtr)(int, int);

   // ==========================================================================
   // Construction
   // ==========================================================================
   HeavenlyTetrarchs(Board* board, int value, int typevalue);

   void InitStandardAttackPatterns();
   void InitIguiCapturePatterns();

   virtual ~HeavenlyTetrarchs();
   
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
      NORTHWEST = 0, NORTHEAST, SOUTHWEST, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 4 };

   // ==========================================================================
   // Data
   // ==========================================================================
   Board* mpBoard;                    // Pointer to the game's board
   int    mWidth;                     // Width of the games' board
   int    mSize;                      // Board size
   Bitboard* mIguiCaptureBitboards;   // Array of bitboards for calculating moves

   // Vector of pointers to Board methods for move directions
   std::vector<DirectionMethodPtr> mDirections;

   // Vector of arrays of Bitboards for calculating standard moves
   std::vector<Bitboard*> mAttackBitboards;
};

#endif

