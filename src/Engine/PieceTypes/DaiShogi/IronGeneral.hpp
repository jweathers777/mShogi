////////////////////////////////////////////////////////////////////////////
// Name:         IronGeneral.hpp
// Description:  Interface for class that represents a Iron General
// Created:      08/28/2004 01:19:53 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __IronGeneral_HPP__
#define __IronGeneral_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class IronGeneral : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   IronGeneral(Board* board, int value, int typevalue);
   virtual ~IronGeneral() {};
};

#endif

