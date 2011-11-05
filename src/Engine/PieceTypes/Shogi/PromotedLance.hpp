////////////////////////////////////////////////////////////////////////////
// Name:         PromotedLance.hpp
// Description:  Interface for class that represents a promoted lance
// Created:      08/31/2004 10:39:12 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __PromotedLance_HPP__
#define __PromotedLance_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class PromotedLance : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   PromotedLance(Board* board, int value, int typevalue);
   virtual ~PromotedLance() {};
};

#endif

