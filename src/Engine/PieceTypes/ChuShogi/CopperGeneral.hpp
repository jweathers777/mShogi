////////////////////////////////////////////////////////////////////////////
// Name:         CopperGeneral.hpp
// Description:  Interface for class that represents a copper general
// Created:      08/31/2004 06:34:30 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __CopperGeneral_HPP__
#define __CopperGeneral_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class CopperGeneral : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   CopperGeneral(Board* board, int value, int typevalue);
   virtual ~CopperGeneral() {};
};

#endif

