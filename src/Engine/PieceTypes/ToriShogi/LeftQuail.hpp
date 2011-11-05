////////////////////////////////////////////////////////////////////////////
// Name:         LeftQuail.hpp
// Description:  Interface for class that represents a Left Quail
// Created:      10/20/2004 08:25:18 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __LeftQuail_HPP__
#define __LeftQuail_HPP__

// mShogi header files
#include "StaticDyamicPatternPiece.hpp"

class LeftQuail : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   LeftQuail(Board* board, int value, int typevalue);
   virtual ~LeftQuail() {};

private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 2 };
};

#endif

