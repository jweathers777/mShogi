////////////////////////////////////////////////////////////////////////////
// Name:         AngryBoar.hpp
// Description:  Interface for class that represents a angry boar
// Created:      10/23/2004 02:09:31 Eastern Daylight Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __AngryBoar_HPP__
#define __AngryBoar_HPP__

// mShogi header files
#include "SymStaticPatternPiece.hpp"

class AngryBoar : public SymStaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   AngryBoar(Board* board, int value, int typevalue);
   virtual ~AngryBoar() {};
};

#endif

