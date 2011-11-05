////////////////////////////////////////////////////////////////////////////
// Name:         SilverGeneral.hpp
// Description:  Interface for class that represents a silver general
// Created:      May 1, 2004 08:16:19 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __SilverGeneral_HPP__
#define __SilverGeneral_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class SilverGeneral : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   SilverGeneral(Board* board, int value, int typevalue);
   virtual ~SilverGeneral() {};
};

#endif
