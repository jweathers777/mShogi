////////////////////////////////////////////////////////////////////////////
// Name:         ViolentOx.hpp
// Description:  Interface for class that represents a violent ox
// Created:      10/23/2004 01:49:42 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __ViolentOx_HPP__
#define __ViolentOx_HPP__

// mShogi header files
#include "LimSymDynPatternPiece.hpp"

class ViolentOx : public LimSymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   ViolentOx(Board* board, int value, int typevalue);
   virtual ~ViolentOx() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { 
      NORTH = 0, WEST, EAST, SOUTH 
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

