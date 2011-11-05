////////////////////////////////////////////////////////////////////////////
// Name:         VerticalMover.hpp
// Description:  Interface for class that represents a vertical mover
// Created:      08/31/2004 10:49:59 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __VerticalMover_HPP__
#define __VerticalMover_HPP__

// mShogi header files
#include "SymStatDynPatternPiece.hpp"

class VerticalMover : public SymStatDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   VerticalMover(Board* board, int value, int typevalue);
   virtual ~VerticalMover() {};
   
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

