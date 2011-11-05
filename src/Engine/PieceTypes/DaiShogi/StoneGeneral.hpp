////////////////////////////////////////////////////////////////////////////
// Name:         StoneGeneral.hpp
// Description:  Interface for class that represents a Stone General
// Created:      10/23/2004 01:37:35 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __StoneGeneral_HPP__
#define __StoneGeneral_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class StoneGeneral : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   StoneGeneral(Board* board, int value, int typevalue);
   virtual ~StoneGeneral() {};
};

#endif

