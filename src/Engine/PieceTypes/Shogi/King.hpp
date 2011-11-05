////////////////////////////////////////////////////////////////////////////
// Name:         King.hpp
// Description:  Interface for class that represents a king
// Created:      May 1, 2004 08:44:20 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __King_HPP__
#define __King_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class King : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   King(Board* board, int value, int typevalue);
   virtual ~King() {};
};

#endif
