////////////////////////////////////////////////////////////////////////////
// Name:         MoveRec.hpp
// Description:  Structure for storing minimal move information
// Created:      07/13/2004 10:15:05 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 02:34:19 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __MoveRec_HPP__
#define __MoveRec_HPP__

struct MoveRec
{
   int mSource;            // Source square
   int mVisited;           // Visited square
   int mDestination;       // Destination square
   void* mPiecePtr;        // Pointer to moving piece
   char mFlags;            // Flags indicating promotion/suicide
};

extern MoveRec gNullMoveRec;

#endif
