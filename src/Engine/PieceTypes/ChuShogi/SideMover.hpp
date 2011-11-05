////////////////////////////////////////////////////////////////////////////
// Name:         SideMover.hpp
// Description:  Interface for class that represents a side mover
// Created:      08/31/2004 10:44:38 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __SideMover_HPP__
#define __SideMover_HPP__

// mShogi header files
#include "SymStatDynPatternPiece.hpp"

class SideMover : public SymStatDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   SideMover(Board* board, int value, int typevalue);
   virtual ~SideMover() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      WEST = 0, EAST
   };

   enum { DIRECTION_COUNT = 2 };
};

#endif

