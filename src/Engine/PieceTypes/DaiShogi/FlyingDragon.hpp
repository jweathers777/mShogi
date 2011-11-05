////////////////////////////////////////////////////////////////////////////
// Name:         FlyingDragon.hpp
// Description:  Interface for class that represents a flying dragon
// Created:      10/23/2004 02:27:29 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FlyingDragon_HPP__
#define __FlyingDragon_HPP__

// mShogi header files
#include "LimSymDynPatternPiece.hpp"

class FlyingDragon : public LimSymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   FlyingDragon(Board* board, int value, int typevalue);
   virtual ~FlyingDragon() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { 
      NORTHWEST = 0, NORTHEAST, SOUTHWEST, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

