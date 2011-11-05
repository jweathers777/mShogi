////////////////////////////////////////////////////////////////////////////
// Name:         StaticDynamicPatternPiece.hpp
// Description:  Interface for class that represents a piece type
//               whose move can be represented with a static bit pattern
//               a dynamic bit pattern
// Created:      10/20/2004 08:02:35 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __StaticDynamicPatternPiece_HPP__
#define __StaticDynamicPatternPiece_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class StaticDynamicPatternPiece : public PieceType
{
public:
   // ==========================================================================
   // Type definitions
   // ==========================================================================
   typedef int (Board::*DirectionMethodPtr)(int, int);

   // ==========================================================================
   // Construction
   // ==========================================================================
   void InitAttackPatterns();
   virtual ~StaticDynamicPatternPiece();
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   bool ThreatensSquare(Piece* piece, int square);
   void GenerateCaptures(std::vector<Move>& movelist, Piece* piece);
   void GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece);
   
protected:
   // ==========================================================================
   // Data
   // ==========================================================================
  
   // Pointer to the game's board
   Board* mpBoard;

   // Board size
   int    mSize;
   
   // Vector of pointers to Board methods for move directions
   std::vector<DirectionMethodPtr> mDirections;

   // Vector of arrays of Bitboards for calculating moves
   std::vector<Bitboard*> mAttackBitboards[2];
   
   // Array of static pattern bitboards for calculating moves
   Bitboard* mStaticAttackBitboards[2];
};

#endif

