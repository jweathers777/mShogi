////////////////////////////////////////////////////////////////////////////
// Name:         SavageTiger.hpp
// Description:  Interface for class that represents a savage tiger
// Created:      10/25/2004 11:07:01 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __SavageTiger_HPP__
#define __SavageTiger_HPP__

// mShogi header files
#include "LimDynamicPatternPiece.hpp"

class SavageTiger : public LimDynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   SavageTiger(Board* board, int value, int typevalue);
   virtual ~SavageTiger() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { 
      NORTHWEST = 0, NORTH, NORTHEAST, SOUTH 
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

