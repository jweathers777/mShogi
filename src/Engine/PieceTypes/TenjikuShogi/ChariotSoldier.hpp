////////////////////////////////////////////////////////////////////////////
// Name:         ChariotSoldier.hpp
// Description:  Interface for class that represents a chariot soldier
// Created:      08/31/2004 06:25:47 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __ChariotSoldier_HPP__
#define __ChariotSoldier_HPP__

// mShogi header files
#include "LimSymDynPatternPiece.hpp"

class ChariotSoldier : public LimSymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   ChariotSoldier(Board* board, int value, int typevalue);
   virtual ~ChariotSoldier() {};
   
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

