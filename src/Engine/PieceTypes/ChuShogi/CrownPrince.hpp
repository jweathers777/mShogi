////////////////////////////////////////////////////////////////////////////
// Name:         CrownPrince.hpp
// Description:  Interface for class that represents a crown prince
// Created:      08/31/2004 09:24:14 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __CrownPrince_HPP__
#define __CrownPrince_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class CrownPrince : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   CrownPrince(Board* board, int value, int typevalue);
   virtual ~CrownPrince() {};
};

#endif

