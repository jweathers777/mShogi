////////////////////////////////////////////////////////////////////////////////
// Name:         DragonKing.hpp
// Description:  Interface for class that represents a DragonKing
// Created:      May 1, 2004 9:40:02 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef __DragonKing_HPP__
#define __DragonKing_HPP__

// mShogi header files
#include "SymStatDynPatternPiece.hpp"

class DragonKing : public SymStatDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   DragonKing(Board* board, int value, int typevalue);
   virtual ~DragonKing() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, WEST, EAST, SOUTH, STATIC
   };
   
   enum { DIRECTION_COUNT = 4 };
};

#endif
