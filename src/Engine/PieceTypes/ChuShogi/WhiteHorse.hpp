////////////////////////////////////////////////////////////////////////////
// Name:         WhiteHorse.hpp
// Description:  Interface for class that represents a white horse
// Created:      08/28/2004 01:44:16 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:07 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __WhiteHorse_HPP__
#define __WhiteHorse_HPP__

// mShogi header files
#include "DynamicPatternPiece.hpp"

class WhiteHorse : public DynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   WhiteHorse(Board* board, int value, int typevalue);
   virtual ~WhiteHorse() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { NORTHWEST = 0, NORTH, NORTHEAST, SOUTH };

   enum { DIRECTION_COUNT = 4 };
};

#endif

