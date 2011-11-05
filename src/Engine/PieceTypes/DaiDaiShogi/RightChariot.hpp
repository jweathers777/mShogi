////////////////////////////////////////////////////////////////////////////
// Name:         RightChariot.hpp
// Description:  Interface for class that represents a Right Chariot
// Created:      10/25/2004 10:50:20 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __RightChariot_HPP__
#define __RightChariot_HPP__

// mShogi header files
#include "StaticDyamicPatternPiece.hpp"

class RightChariot : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   RightChariot(Board* board, int value, int typevalue);
   virtual ~RightChariot() {};

private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, NORTHEAST, SOUTHWEST
   };

   enum { DIRECTION_COUNT = 3 };
};

#endif

