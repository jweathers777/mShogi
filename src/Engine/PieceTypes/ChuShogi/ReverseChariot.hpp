////////////////////////////////////////////////////////////////////////////
// Name:         ReverseChariot.hpp
// Description:  Interface for class that represents a reverse chariot
// Created:      08/28/2004 01:24:38 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __ReverseChariot_HPP__
#define __ReverseChariot_HPP__

// mShogi header files
#include "SymDynPatternPiece.hpp"

class ReverseChariot : public SymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   ReverseChariot(Board* board, int value, int typevalue);
   virtual ~ReverseChariot() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, SOUTH
   };

   enum { DIRECTION_COUNT = 2 };
};

#endif

