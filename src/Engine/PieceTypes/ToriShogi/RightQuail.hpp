////////////////////////////////////////////////////////////////////////////
// Name:         RightQuail.hpp
// Description:  Interface for class that represents a Right Quail
// Created:      10/20/2004 01:28:19 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __RightQuail_HPP__
#define __RightQuail_HPP__

// mShogi header files
#include "StaticDyamicPatternPiece.hpp"

class RightQuail : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   RightQuail(Board* board, int value, int typevalue);
   virtual ~RightQuail() {};

private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, SOUTHWEST
   };

   enum { DIRECTION_COUNT = 2 };
};

#endif

