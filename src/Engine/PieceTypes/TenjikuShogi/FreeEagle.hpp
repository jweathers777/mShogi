////////////////////////////////////////////////////////////////////////////
// Name:         FreeEagle.hpp
// Description:  Interface for class that represents a free eagle
// Created:      08/31/2004 09:47:54 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FreeEagle_HPP__
#define __FreeEagle_HPP__

// mShogi header files
#include "SymStatDynPatternPiece.hpp"

class FreeEagle : public SymStatDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   FreeEagle(Board* board, int value, int typevalue);
   virtual ~FreeEagle() {};
   
private:
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

