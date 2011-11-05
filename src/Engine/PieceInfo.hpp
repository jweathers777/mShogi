////////////////////////////////////////////////////////////////////////////
// Name:         PieceInfo.hpp
// Description:  Definition of the piece information structure
// Created:      08/19/2004 02:57:46 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 03:20:37 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __PieceInfo_HPP__
#define __PieceInfo_HPP__

struct PieceInfo
{
   int mType;
   int mPromoted;
   int mSide;
};

enum { NO_PIECE = -1 };

#endif

