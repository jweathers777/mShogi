////////////////////////////////////////////////////////////////////////////
// Name:         GoldGeneral.hpp
// Description:  Interface for class that represents a Gold General
// Created:      May 1, 2004 08:30:22 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __GoldGeneral_HPP__
#define __GoldGeneral_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class GoldGeneral : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   GoldGeneral(Board* board, int value, int typevalue);
   virtual ~GoldGeneral() {};
};

#endif
