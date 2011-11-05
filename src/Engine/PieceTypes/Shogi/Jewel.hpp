////////////////////////////////////////////////////////////////////////////
// Name:         Jewel.hpp
// Description:  Interface for class that represents a Jewel
// Created:      May 1, 2004 09:00:08 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Jewel_HPP__
#define __Jewel_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class Jewel : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Jewel(Board* board, int value, int typevalue);
   virtual ~Jewel() {};
};

#endif

