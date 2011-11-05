////////////////////////////////////////////////////////////////////////////
// Name:         FlyingOx.hpp
// Description:  Interface for class that represents a flying ox
// Created:      08/31/2004 09:33:46 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FlyingOx_HPP__
#define __FlyingOx_HPP__

// mShogi header files
#include "SymDynPatternPiece.hpp"

class FlyingOx : public SymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   FlyingOx(Board* board, int value, int typevalue);
   virtual ~FlyingOx() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTHWEST = 0, NORTH, NORTHEAST, SOUTHWEST, SOUTH, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 6 };
};

#endif

