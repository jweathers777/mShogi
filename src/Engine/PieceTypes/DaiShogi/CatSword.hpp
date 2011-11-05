////////////////////////////////////////////////////////////////////////////
// Name:         CatSword.hpp
// Description:  Interface for class that represents a cat sword
// Created:      10/23/2004 01:42:17 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __CatSword_HPP__
#define __CatSword_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class CatSword : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   CatSword(Board* board, int value, int typevalue);
   virtual ~CatSword() {};
};

#endif

