////////////////////////////////////////////////////////////////////////////
// Name:         Lion.hpp
// Description:  Interface for class that represents a lion
// Created:      09/01/2004 09:23:56 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Lion_HPP__
#define __Lion_HPP__

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "PieceType.hpp"
#include "common.hpp"

// Forward declarations
class Board;

class Lion : public PieceType
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Lion(Board* board, int value, int typevalue);
   virtual ~Lion();
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   bool ThreatensSquare(Piece* piece, int square);
   void GenerateCaptures(std::vector<Move>& movelist, Piece* piece);
   void GenerateNonCaptures(std::vector<Move>& movelist, Piece* piece);
   
   // ==========================================================================
   // Class Variables
   // ==========================================================================
   static bool sLionCaptureRules;
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { TYPE_A = 0, TYPE_B };

   // ==========================================================================
   // Data
   // ==========================================================================
   Board* mpBoard;                 // Pointer to the game's board
   int    mSize;                   // Board size
   Bitboard* mAttackBitboards[2];  // Array of bitboards for calculating moves
};

#endif

