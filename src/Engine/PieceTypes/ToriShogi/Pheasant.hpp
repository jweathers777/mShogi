////////////////////////////////////////////////////////////////////////////
// Name:         Pheasant.hpp
// Description:  Interface for class that represents a Pheasant
// Created:      10/20/2004 12:02:31 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Pheasant_HPP__
#define __Pheasant_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class Pheasant : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Pheasant(Board* board, int value, int typevalue);
   virtual ~Pheasant() {};
};

#endif

