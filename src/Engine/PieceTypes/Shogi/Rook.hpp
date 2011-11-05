////////////////////////////////////////////////////////////////////////////////
// Name:         Rook.hpp
// Description:  Interface for class that represents a Rook
// Created:      May 1, 2004 11:59:12 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef __Rook_HPP__
#define __Rook_HPP__

// mShogi header files
#include "SymDynPatternPiece.hpp"

class Rook : public SymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Rook(Board* board, int value, int typevalue);
   virtual ~Rook() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, WEST, EAST, SOUTH
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif
