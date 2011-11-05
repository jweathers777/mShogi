////////////////////////////////////////////////////////////////////////////
// Name:         JumpingGeneral.hpp
// Description:  Interface for class that represents a piece type 
//               whose move can be represented with a symmetric 
//               dynamic bit pattern and whose movement powers include
//               the ability to jump other pieces of lesser rank
// Created:      08/31/2004 05:45:23 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:22:10 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __JumpingGeneral_HPP__
#define __JumpingGeneral_HPP__

// STL header files
#include <map>

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class JumpingGeneral : public PieceType
{
public:
   // ==========================================================================
   // Type definitions
   // ==========================================================================
   typedef int (Board::*DirectionMethodPtr)(int, int);

   // ==========================================================================
   // Construction
   // ==========================================================================
   virtual void InitAttackPatterns();
   virtual ~JumpingGeneral();
   
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

   // Jumping general rank
   int    mRank; 

   // Pointer to game's piece rank map
   std::map<int,int>* mpRankMap; 
   
   // Vector of pointers to Board methods for move directions
   std::vector<DirectionMethodPtr> mDirections;

   // Vector of arrays of Bitboards for calculating moves
   std::vector<Bitboard*> mAttackBitboards;
};

#endif

