////////////////////////////////////////////////////////////////////////////
// Name:         PromotedSilver.hpp
// Description:  Interface for class that represents a Promoted Silver
// Created:      May 1, 2004 08:36:32 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __PromotedSilver_HPP__
#define __PromotedSilver_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class PromotedSilver : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   PromotedSilver(Board* board, int value, int typevalue);
   virtual ~PromotedSilver() {};
};

#endif
