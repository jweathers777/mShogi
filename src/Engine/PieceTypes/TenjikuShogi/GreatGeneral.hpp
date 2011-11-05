////////////////////////////////////////////////////////////////////////////
// Name:         GreatGeneral.hpp
// Description:  Interface for class that represents a great general
// Created:      08/31/2004 10:00:47 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __GreatGeneral_HPP__
#define __GreatGeneral_HPP__

// mShogi header files
#include "JumpingGeneral.hpp"

class GreatGeneral : public JumpingGeneral
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   GreatGeneral(Board* board, int value, int typevalue, int rank,
                 std::map<int,int>* rankmap);
   virtual ~GreatGeneral() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTHWEST = 0, NORTH, NORTHEAST, WEST, 
      EAST, SOUTHWEST, SOUTH, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 8 };
};

#endif

