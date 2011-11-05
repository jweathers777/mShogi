////////////////////////////////////////////////////////////////////////////
// Name:         BishopGeneral.hpp
// Description:  Interface for class that represents a bishop general
// Created:      08/31/2004 06:15:27 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:23:06 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __BishopGeneral_HPP__
#define __BishopGeneral_HPP__

// mShogi header files
#include "JumpingGeneral.hpp"

class BishopGeneral : public JumpingGeneral
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   BishopGeneral(Board* board, int value, int typevalue, int rank,
                 std::map<int,int>* rankmap);
   virtual ~BishopGeneral() {};
   
   // ==========================================================================
   // Constants
   // ==========================================================================
   enum {
      NORTHWEST = 0, NORTHEAST, SOUTHWEST, SOUTHEAST
   };

   enum { DIRECTION_COUNT = 4 };
};

#endif

