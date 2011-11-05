////////////////////////////////////////////////////////////////////////////
// Name:         ViolentBear.hpp
// Description:  Interface for class that represents a violent bear
// Created:      10/25/2004 11:26:58 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __ViolentBear_HPP__
#define __ViolentBear_HPP__

// mShogi header files
#include "LimDynamicPatternPiece.hpp"

class ViolentBear : public LimDynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   ViolentBear(Board* board, int value, int typevalue);
   virtual ~ViolentBear() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { 
      NORTHWEST = 0, NORTHEAST, WEST, EAST
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

