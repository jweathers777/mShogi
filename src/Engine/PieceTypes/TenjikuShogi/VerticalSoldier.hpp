////////////////////////////////////////////////////////////////////////////
// Name:         VerticalSoldier.hpp
// Description:  Interface for class that represents a vertical soldier
// Created:      09/01/2004 09:20:05 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __VerticalSoldier_HPP__
#define __VerticalSoldier_HPP__

// mShogi header files
#include "LimDynamicPatternPiece.hpp"

class VerticalSoldier : public LimDynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   VerticalSoldier(Board* board, int value, int typevalue);
   virtual ~VerticalSoldier() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, WEST, EAST, SOUTH
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

