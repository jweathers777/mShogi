////////////////////////////////////////////////////////////////////////////
// Name:         FreeKing.hpp
// Description:  Interface for class that represents a free king
// Created:      08/31/2004 09:55:17 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FreeKing_HPP__
#define __FreeKing_HPP__

// mShogi header files
#include "SymDynPatternPiece.hpp"

class FreeKing : public SymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   FreeKing(Board* board, int value, int typevalue);
   virtual ~FreeKing() {};
   
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

