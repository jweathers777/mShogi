////////////////////////////////////////////////////////////////////////////
// Name:         HowlingDog.hpp
// Description:  Interface for class that represents a howling dog
// Created:      10/23/2004 02:40:48 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __HowlingDog_HPP__
#define __HowlingDog_HPP__

// mShogi header files
#include "StaticDyamicPatternPiece.hpp"

class HowlingDog : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   HowlingDog(Board* board, int value, int typevalue);
   virtual ~HowlingDog() {};

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

