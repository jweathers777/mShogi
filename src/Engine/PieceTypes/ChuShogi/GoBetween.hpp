////////////////////////////////////////////////////////////////////////////
// Name:         GoBetween.hpp
// Description:  Interface for class that represents a GoBetween 
// Created:      08/28/2004 12:52:45 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __GoBetween_HPP__
#define __GoBetween_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class GoBetween : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   GoBetween(Board* board, int value, int typevalue);
   virtual ~GoBetween() {};
};

#endif

