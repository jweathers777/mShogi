////////////////////////////////////////////////////////////////////////////
// Name:         Whale.hpp
// Description:  Interface for class that represents a whale
// Created:      08/28/2004 04:09:23 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Whale_HPP__
#define __Whale_HPP__

// mShogi header files
#include "DynamicPatternPiece.hpp"

class Whale : public DynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Whale(Board* board, int value, int typevalue);
   virtual ~Whale() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { NORTH = 0, SOUTHWEST, SOUTH, SOUTHEAST };

   enum { DIRECTION_COUNT = 4 };
};

#endif

