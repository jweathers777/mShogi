////////////////////////////////////////////////////////////////////////////
// Name:         MultiGeneral.hpp
// Description:  Interface for class that represents a multi-general
// Created:      08/31/2004 10:20:26 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __MultiGeneral_HPP__
#define __MultiGeneral_HPP__

// mShogi header files
#include "DynamicPatternPiece.hpp"

class MultiGeneral : public DynamicPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   MultiGeneral(Board* board, int value, int typevalue);
   virtual ~MultiGeneral() {};
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum { NORTH = 0, SOUTHWEST, SOUTHEAST };

   enum { DIRECTION_COUNT = 3 };
};

#endif

