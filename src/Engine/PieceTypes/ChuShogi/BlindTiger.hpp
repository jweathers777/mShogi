////////////////////////////////////////////////////////////////////////////
// Name:         BlindTiger.hpp
// Description:  Interface for class that represents a blind tiger
// Created:      08/31/2004 06:22:08 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __BlindTiger_HPP__
#define __BlindTiger_HPP__

// mShogi header files
#include "StaticPatternPiece.hpp"

class BlindTiger : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   BlindTiger(Board* board, int value, int typevalue);
   virtual ~BlindTiger() {};
};

#endif

