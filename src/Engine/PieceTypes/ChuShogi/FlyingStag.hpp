////////////////////////////////////////////////////////////////////////////
// Name:         FlyingStag.hpp
// Description:  Interface for class that represents a flying stag
// Created:      08/31/2004 09:38:14 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FlyingStag_HPP__
#define __FlyingStag_HPP__

// mShogi header files
#include "SymStatDynPatternPiece.hpp"

class FlyingStag : public SymStatDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   FlyingStag(Board* board, int value, int typevalue);
   virtual ~FlyingStag() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, SOUTH
   };

   enum { DIRECTION_COUNT = 2 };
};

#endif

