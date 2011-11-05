////////////////////////////////////////////////////////////////////////////
// Name:         Lance.hpp
// Description:  Interface for class that represents a lance
// Created:      08/28/2004 01:32:16 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Lance_HPP__
#define __Lance_HPP__

// mShogi header files
#include "DynamicPatternPiece.hpp"

class Lance : public DynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Lance(Board* board, int value, int typevalue);
   virtual ~Lance() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { NORTH = 0 };

   enum { DIRECTION_COUNT = 1 };
};

#endif

