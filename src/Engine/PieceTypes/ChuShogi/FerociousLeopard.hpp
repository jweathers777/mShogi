////////////////////////////////////////////////////////////////////////////
// Name:         FerociousLeopard.hpp
// Description:  Interface for class that represents a ferocious leopard
// Created:      08/31/2004 09:30:29 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FerociousLeopard_HPP__
#define __FerociousLeopard_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class FerociousLeopard : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   FerociousLeopard(Board* board, int value, int typevalue);
   virtual ~FerociousLeopard() {};
};

#endif

