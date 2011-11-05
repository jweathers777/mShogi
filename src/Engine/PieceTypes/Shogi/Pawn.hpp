////////////////////////////////////////////////////////////////////////////////
// Name:         Pawn.hpp
// Description:  Interface for class that represents a Pawn
// Created:      May 1, 2004 12:37:51 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef PAWN_HPP
#define PAWN_HPP

// mShogi header files
#include "StaticPatternPiece.hpp"

class Pawn : public StaticPatternPiece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Pawn(Board* board, int value, int typevalue);
   virtual ~Pawn() {};
};

#endif // PAWN_HPP
