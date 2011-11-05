////////////////////////////////////////////////////////////////////////////
// Name:         ViceGeneral.hpp
// Description:  Interface for class that represents a vice general
// Created:      08/29/2004 09:03:12 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __ViceGeneral_HPP__
#define __ViceGeneral_HPP__

// STL header files
#include <map>

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "AreaMover.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class ViceGeneral : public PieceType
{
public:
   // ==========================================================================
   // Type definitions
   // ==========================================================================
   typedef int (Board::*DirectionMethodPtr)(int, int);

   // ==========================================================================
   // Construction
   // ==========================================================================
   ViceGeneral(Board* board, int value, int typevalue, int rank,
               std::map<int,int>* rankmap);
   virtual ~ViceGeneral();
   
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
   int    mRank;     // Jumping general rank

   std::map<int,int>* mpRankMap; // Pointer to game's piece rank map
   
   // Directional constants
   enum {
      NORTHWEST = 0, NORTHEAST, SOUTHWEST, SOUTHEAST
   };

   enum {
      DIRECTIONS = 4
   };

   // Directional method pointers
   DirectionMethodPtr mDirections[DIRECTIONS];
   
   // Array of attack bitboards for calculating area moves
   Bitboard* mAreaAttackBitboards; 
                                         
   // Pointer to the instance of the area mover object
   AreaMover* mpAreaMover;

   Bitboard* mAttackBitboards[DIRECTIONS];  // Array of bitboards for calculating moves
};

#endif

