////////////////////////////////////////////////////////////////////////////////
// Name:         Piece.hpp
// Description:  Interface for a class that represents a piece
// Created:      Apr 20, 2004 3:10:49 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef PIECE_HPP
#define PIECE_HPP

// STL header files
#include <vector>

// mShogi header files
#include "Move.hpp"
#include "PieceType.hpp"

class Piece
{
public:
   // ==========================================================================
   // Construction
   // ==========================================================================
   Piece(PieceType* type, int color, int square, PieceType* promoted=0);
   
   virtual ~Piece();
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   void GenerateCaptures(std::vector<Move>& movelist)
   {
      mpType->GenerateCaptures(movelist, this);
   }
   void GenerateNonCaptures(std::vector<Move>& movelist)
   {
      mpType->GenerateNonCaptures(movelist, this);
   }
   
   // ==========================================================================
	// Data
	// ==========================================================================
   int mColor;     // The piece's color
   int mSquare;    // The piece's position on the board
   
   int mValue;     // For quick reference
   int mTypeValue; // Value that identifies the piece type 
                   // for component calculation

   bool mCaptured; // True when the piece is captured
   bool mPromoted; // True when the piece is in its promoted form
   
   PieceType* mpType;  // Type of piece
   
   PieceType* mpRelatedType; // Promoted type for unpromoted pieces and
                             // unpromoted type for promoted pieces
};

#endif // PIECE_HPP
