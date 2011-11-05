////////////////////////////////////////////////////////////////////////////
// Name:         PromotedKnight.hpp
// Description:  Interface for class that represents a promoted knight
// Created:      08/31/2004 10:29:29 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __PromotedKnight_HPP__
#define __PromotedKnight_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class PromotedKnight : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   PromotedKnight(Board* board, int value, int typevalue);
   virtual ~PromotedKnight() {};
};

#endif

