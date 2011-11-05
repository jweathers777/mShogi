////////////////////////////////////////////////////////////////////////////
// Name:         FreeBoar.hpp
// Description:  Interface for class that represents a free boar
// Created:      08/31/2004 09:44:10 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __FreeBoar_HPP__
#define __FreeBoar_HPP__

// mShogi header files
#include "SymDynPatternPiece.hpp"

class FreeBoar : public SymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   FreeBoar(Board* board, int value, int typevalue);
   virtual ~FreeBoar() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTHWEST = 0, NORTHEAST, WEST, EAST, SOUTHWEST, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 6 };
};

#endif

