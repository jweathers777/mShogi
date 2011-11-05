////////////////////////////////////////////////////////////////////////////
// Name:         EvilWolf.hpp
// Description:  Interface for class that represents a evil wolf
// Created:      10/23/2004 02:23:46 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __EvilWolf_HPP__
#define __EvilWolf_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class EvilWolf : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   EvilWolf(Board* board, int value, int typevalue);
   virtual ~EvilWolf() {};
};

#endif

