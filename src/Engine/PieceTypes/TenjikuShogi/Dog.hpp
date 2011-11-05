////////////////////////////////////////////////////////////////////////////
// Name:         Dog.hpp
// Description:  Interface for class that represents a Dog
// Created:      08/28/2004 12:58:50 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Dog_HPP__
#define __Dog_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class Dog : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Dog(Board* board, int value, int typevalue);
   virtual ~Dog() {};
};

#endif

