////////////////////////////////////////////////////////////////////////////
// Name:         SideSoldier.hpp
// Description:  Interface for class that represents a side soldier
// Created:      09/01/2004 09:06:26 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __SideSoldier_HPP__
#define __SideSoldier_HPP__

// mShogi header files
#include "LimDynamicPatternPiece.hpp"

class SideSoldier : public LimDynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   SideSoldier(Board* board, int value, int typevalue);
   virtual ~SideSoldier() {};
   
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

