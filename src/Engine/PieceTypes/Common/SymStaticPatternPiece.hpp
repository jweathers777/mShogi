////////////////////////////////////////////////////////////////////////////
// Name:         SymStaticPatternPiece.hpp
// Description:  Interface for class that represents a piece type whose
//               move can be represented with a symmetric static bit pattern
// Created:      08/31/2004 11:55:18 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:22:10 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __SymStaticPatternPiece_HPP__
#define __SymStaticPatternPiece_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class SymStaticPatternPiece : public PieceType
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   virtual ~SymStaticPatternPiece();
   
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
   Board* mpBoard;              // Pointer to the game's board
   int    mSize;                // Board size
   Bitboard* mAttackBitboards;  // Array of bitboards for calculating moves
};

#endif

