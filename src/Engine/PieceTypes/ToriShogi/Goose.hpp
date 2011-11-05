////////////////////////////////////////////////////////////////////////////
// Name:         Goose.hpp
// Description:  Interface for class that represents a Goose
// Created:      10/20/2004 01:21:21 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Goose_HPP__
#define __Goose_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class Goose : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Goose(Board* board, int value, int typevalue);
   virtual ~Goose() {};
};

#endif

