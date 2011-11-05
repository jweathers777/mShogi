////////////////////////////////////////////////////////////////////////////
// Name:         Tokin.hpp
// Description:  Interface for class that represents a tokin
// Created:      May 1, 2004 08:36:32 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Tokin_HPP__
#define __Tokin_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class Tokin : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Tokin(Board* board, int value, int typevalue);
   virtual ~Tokin() {};
};

#endif
