////////////////////////////////////////////////////////////////////////////
// Name:         RookGeneral.hpp
// Description:  Interface for class that represents a rook general
// Created:      08/31/2004 10:41:31 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __RookGeneral_HPP__
#define __RookGeneral_HPP__

// mShogi header files
#include "JumpingGeneral.hpp"

class RookGeneral : public JumpingGeneral
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   RookGeneral(Board* board, int value, int typevalue, int rank,
                 std::map<int,int>* rankmap);
   virtual ~RookGeneral() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTH = 0, WEST, EAST, SOUTH
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

