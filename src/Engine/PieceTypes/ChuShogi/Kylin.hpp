////////////////////////////////////////////////////////////////////////////
// Name:         Kylin.hpp
// Description:  Interface for class that represents a kylin
// Created:      08/31/2004 10:15:28 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Kylin_HPP__
#define __Kylin_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class Kylin : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Kylin(Board* board, int value, int typevalue);
   virtual ~Kylin() {};
};

#endif

