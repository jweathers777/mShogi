////////////////////////////////////////////////////////////////////////////
// Name:         MoveRec.cpp
// Description:  Structure for storing minimal move information
// Created:      07/14/2004 09:22:25 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 02:34:19 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// =========================================================================
// Header file includes
// =========================================================================

// Standard C++ header files
#include <iostream>

// Local header files
#include "MoveRec.hpp"

using std::ostream;

// Global MoveRec instance initialization

MoveRec gNullMoveRec = {
   -1, // mSource
   -1, // mVisited
   -1, // mDestination
    0, // mPiecePtr
    0  // mFlags
};

// Overload output operator for MoveRec structures
ostream& operator <<(ostream& output, const MoveRec& moverec)
{
   output << moverec.mSource << "->";
   if (moverec.mVisited != -1) output << moverec.mVisited << "->";
   output << moverec.mDestination << " (";
   output << moverec.mPiecePtr << ", " << moverec.mFlags << ")";

   return output;
}
