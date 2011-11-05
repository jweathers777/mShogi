////////////////////////////////////////////////////////////////////////////
// Name:         Knight.hpp
// Description:  Interface for class that represents a Knight
// Created:      08/28/2004 01:04:47 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Knight_HPP__
#define __Knight_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class Knight : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Knight(Board* board, int value, int typevalue);
   virtual ~Knight() {};
};

#endif

