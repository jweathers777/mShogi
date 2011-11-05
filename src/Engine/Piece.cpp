////////////////////////////////////////////////////////////////////////////////
// Name:         Piece.cpp
// Description:  Implementation for a class that represents a piece
// Created:      Apr 30, 2004 11:04:09 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// STL headers
#include <vector>

// mShogi headers
#include "common.hpp"
#include "Piece.hpp"
#include "PieceType.hpp"
#include "Move.hpp"

//--------------------------------------------------------------------------
//       Class:  Piece
//      Method:  Piece
// Description:  Constructs an instance of a piece
//--------------------------------------------------------------------------
Piece::Piece(PieceType* type, int color, int square, PieceType* promoted)
{
   mpType = type;
   mpRelatedType = promoted;
      
   mColor = color;
   mSquare = square;
      
   mValue = mpType->mValue;
   mTypeValue = mpType->mTypeValue;

   mCaptured = false;
   mPromoted = false;
}

//--------------------------------------------------------------------------
//       Class:  Piece
//      Method:  ~Piece
// Description:  Destroys this instance of a piece
//--------------------------------------------------------------------------
Piece::~Piece()
{
}
