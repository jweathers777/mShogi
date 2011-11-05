////////////////////////////////////////////////////////////////////////////
// Name:         Swallow.hpp
// Description:  Interface for class that represents a Swallow
// Created:      10/20/2004 01:17:51 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Swallow_HPP__
#define __Swallow_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class Swallow : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Swallow(Board* board, int value, int typevalue);
   virtual ~Swallow() {};
};

#endif

