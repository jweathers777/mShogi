////////////////////////////////////////////////////////////////////////////
// Name:         AreaMover.hpp
// Description:  A class that represents the area-move capabilities 
//               of a piece that can make 3-step area moves
// Created:      08/29/2004 03:37:46 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __AreaMover_HPP__
#define __AreaMover_HPP__

// STL header files
#include <vector>

// Boost header files
#include <boost/dynamic_bitset.hpp>

// mShogi header files
#include "common.hpp"

// Forward declarations
class Board;

class AreaMover
{
private:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   AreaMover(Board* board);

public:
   static AreaMover* GetInstance(Board* board);
   ~AreaMover();

   // ======================================================================
   // Operations
   // ======================================================================   
   bool HasLegalPath(int start, int finish);

private:
   void InitAttackBitboards();
   void InitAreaPaths();
   void ShiftCenter(Bitboard& bitboard, int oldcenter, int newcenter);

   // ======================================================================
   // Data
   // ======================================================================
   int mWidth, mHeight;        // Dimensions of the board
   int mSize;                  // Size of the board
   Board* mpBoard;             // Pointer to the board

public:   
   // Array of attack bitboards for calculating area moves
   Bitboard* mAttackBitboards; 
                                         
   // Arrays of path bitboard patterns used to calculate 
   // whether a path exists between two points
   std::vector<Bitboard>** mAreaPaths;

private:
   // ======================================================================
   // Static Data
   // ======================================================================
   static AreaMover* pInstance;
};

#endif

