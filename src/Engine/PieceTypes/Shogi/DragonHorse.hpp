////////////////////////////////////////////////////////////////////////////////
// Name:         DragonHorse.hpp
// Description:  Interface for class that represents a dragon horse
// Created:      May 1, 2004 9:40:02 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef __DragonHorse_HPP__
#define __DragonHorse_HPP__

// mShogi header files
#include "SymStatDynPatternPiece.hpp"

class DragonHorse : public SymStatDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   DragonHorse(Board* board, int value, int typevalue);
   virtual ~DragonHorse() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTHWEST = 0, NORTHEAST, SOUTHWEST, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif
