////////////////////////////////////////////////////////////////////////////
// Name:         FireDemon.hpp
// Description:  Interface for class that represents a fire demon
// Created:      09/02/2004 11:24:56 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FireDemon_HPP__
#define __FireDemon_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "AreaMover.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class FireDemon : public PieceType
{
public:
   // ==========================================================================
   // Type definitions
   // ==========================================================================
   typedef int (Board::*DirectionMethodPtr)(int, int);

   // ==========================================================================
   // Construction
   // ==========================================================================
   FireDemon(Board* board, int value, int typevalue);
   
   void InitStandardAttackPatterns();
   void InitBurnPatterns();
   
   virtual ~FireDemon();
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   bool ThreatensSquare(Piece* piece, int square);
   void GenerateCaptures(std::vector<Move>& movelist, Piece* piece);
   void GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece);
   
private:
   // ==========================================================================
   // Data
   // ==========================================================================
   Board* mpBoard;   // Pointer to the game's board
   int    mSize;     // Board size
   
   // Directional constants
   enum {
      NORTHWEST = 0, NORTH, NORTHEAST, SOUTHWEST, SOUTH, SOUTHEAST
   };

   enum {
      DIRECTION_COUNT = 6
   };

   // Directional method pointers
   std::vector<DirectionMethodPtr> mDirections;
   
   // Array of attack bitboards for calculating area moves
   Bitboard* mAreaAttackBitboards; 
                                         
   // Pointer to the instance of the area mover object
   AreaMover* mpAreaMover;

   // Array of bitboards for calculating moves
   std::vector<Bitboard*> mAttackBitboards;

public:
   // Array of bitboards for calculating burn captures
   Bitboard* mBurnBitboards; 
};

#endif

