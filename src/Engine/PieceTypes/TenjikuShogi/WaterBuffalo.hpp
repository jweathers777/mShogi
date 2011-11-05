////////////////////////////////////////////////////////////////////////////
// Name:         WaterBuffalo.hpp
// Description:  Interface for class that represents a water buffalo
// Created:      08/28/2004 04:14:31 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __WaterBuffalo_HPP__
#define __WaterBuffalo_HPP__

// mShogi header files
#include "LimSymDynPatternPiece.hpp"

class WaterBuffalo : public LimSymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   WaterBuffalo(Board* board, int value, int typevalue);
   virtual ~WaterBuffalo() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { 
      NORTHWEST = 0, NORTH, NORTHEAST, WEST, 
      EAST, SOUTHWEST, SOUTH, SOUTHEAST 
   };

   enum { DIRECTION_COUNT = 8 };
};

#endif

