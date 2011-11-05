////////////////////////////////////////////////////////////////////////////////
// Name:         Bishop.hpp
// Description:  Interface for class that represents a bishop
// Created:      May 1, 2004 9:08:02 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef __Bishop_HPP__
#define __Bishop_HPP__

// mShogi header files
#include "SymDynPatternPiece.hpp"

class Bishop : public SymDynPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Bishop(Board* board, int value, int typevalue);
   virtual ~Bishop() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTHWEST = 0, NORTHEAST, SOUTHWEST, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif
