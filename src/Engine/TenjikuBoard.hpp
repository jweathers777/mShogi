////////////////////////////////////////////////////////////////////////////
// Name:         TenjikuBoard.hpp
// Description:  Interface for a class that represents a tenjiku shogi board
// Created:      09/10/2004 07:14:52 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:17:32 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __TenjikuBoard_HPP__
#define __TenjikuBoard_HPP__

// STL header files
#include <vector>

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "common.hpp"
#include "PieceInfo.hpp"
#include "Board.hpp"

// Forward declarations
class Piece;
class Move;

class TenjikuBoard : public Board
{
public:
   // ==========================================================================
	// Constructors and Destructors
	// ==========================================================================
	TenjikuBoard(int width, int height, int zonerow, int piecetypes);
   
   // ==========================================================================
	// Operations
	// ==========================================================================
   uint64 MakeMove(Move* move);
   uint64 UnmakeMove(Move* move);

   void UpdateBurnZones();
   
   // ==========================================================================
	// Data
	// ==========================================================================
   std::vector<Piece*> mpFireDemons[2]; // Pointers to the fire demons
   int                 mFireDemonValue; // Type value of a fire demon
   Bitboard*           mpBurnBitboards; // Pointer to the burn bitboard patterns
   Bitboard            mBurnZones[2];   // Bitboards that reflect all squares
                                        // adjacent to an enemy fire demon
};

#endif

