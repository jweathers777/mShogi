////////////////////////////////////////////////////////////////////////////
// Name:         DrunkElephant.hpp
// Description:  Interface for class that represents a drunk elephant
// Created:      08/31/2004 09:26:59 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __DrunkElephant_HPP__
#define __DrunkElephant_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class DrunkElephant : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   DrunkElephant(Board* board, int value, int typevalue);
   virtual ~DrunkElephant() {};
};

#endif

