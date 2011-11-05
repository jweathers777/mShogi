////////////////////////////////////////////////////////////////////////////
// Name:         RunningChariot.hpp
// Description:  Interface for class that represents a running chariot
// Created:      10/25/2004 11:15:50 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __RunningChariot_HPP__
#define __RunningChariot_HPP__

// mShogi header files
#include "StaticDyamicPatternPiece.hpp"

class RunningChariot : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   RunningChariot(Board* board, int value, int typevalue);
   virtual ~RunningChariot() {};

private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, WEST, EAST, SOUTH
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

