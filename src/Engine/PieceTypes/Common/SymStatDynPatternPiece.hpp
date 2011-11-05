////////////////////////////////////////////////////////////////////////////
// Name:         SymStatDynPatternPiece.hpp
// Description:  Interface for class that represents a piece type
//               whose move can be represented with a symmetric 
//               static bit pattern and a symmetric dynamic bit pattern
// Created:      08/31/2004 03:28:44 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:22:10 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __SymStatDynPatternPiece_HPP__
#define __SymStatDynPatternPiece_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class SymStatDynPatternPiece : public PieceType
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
   virtual ~SymStatDynPatternPiece();
   
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
   std::vector<Bitboard*> mAttackBitboards;
   
   // Array of static pattern bitboards for calculating moves
   Bitboard* mStaticAttackBitboards;
};

#endif

