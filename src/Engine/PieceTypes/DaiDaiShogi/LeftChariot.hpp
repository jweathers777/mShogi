////////////////////////////////////////////////////////////////////////////
// Name:         LeftChariot.hpp
// Description:  Interface for class that represents a Left Chariot
// Created:      10/25/2004 08:54:07 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __LeftChariot_HPP__
#define __LeftChariot_HPP__

// mShogi header files
#include "StaticDyamicPatternPiece.hpp"

class LeftChariot : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   LeftChariot(Board* board, int value, int typevalue);
   virtual ~LeftChariot() {};

private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTHWEST = 0, NORTH, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 3 };
};

#endif

