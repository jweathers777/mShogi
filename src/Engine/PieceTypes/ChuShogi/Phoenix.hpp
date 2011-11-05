////////////////////////////////////////////////////////////////////////////
// Name:         Phoenix.hpp
// Description:  Interface for class that represents a phoenix
// Created:      08/31/2004 10:24:24 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Phoenix_HPP__
#define __Phoenix_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class Phoenix : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Phoenix(Board* board, int value, int typevalue);
   virtual ~Phoenix() {};
};

#endif

