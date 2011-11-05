////////////////////////////////////////////////////////////////////////////////
// Name:         Evaluator.cpp
// Description:  Implementation for a class that evaluates positions in a game
// Created:      Apr 26, 2004 8:51:06 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// STL header files
#include <vector>

// mShogi header files
#include "Evaluator.hpp"
#include "Piece.hpp"

using std::vector;

//--------------------------------------------------------------------------
//       Class:  Evaluator
//      Method:  Evaluator
// Description:  Constructs an instance of an evaluator
//--------------------------------------------------------------------------
Evaluator::Evaluator(vector<Piece>* pieces)
{
   mpPieces = pieces;
}

//--------------------------------------------------------------------------
//       Class:  Evaluator
//      Method:  ~Evaluator
// Description:  Destroys this instance of an evaluator
//--------------------------------------------------------------------------
Evaluator::~Evaluator()
{
}

//--------------------------------------------------------------------------
//       Class:  Evaluator
//      Method:  FullEvaluation
// Description:  Fully evaluates the position of a board
//--------------------------------------------------------------------------
int Evaluator::FullEvaluation(int side)
{
   unsigned int i;
   int value = 0;
   int enemy = 1 - side;
   Piece* ptrPiece;

   // Calculate material value and positional value  
   for (i = 0; i < mpPieces[side].size(); i++) {
      ptrPiece = &(mpPieces[side][i]);
      if (!ptrPiece->mCaptured) {
         value += ptrPiece->mValue + 
                  ptrPiece->mpType->mWeights[side][ptrPiece->mSquare];
      }
   }

   for (i = 0; i < mpPieces[enemy].size(); i++) {
      ptrPiece = &(mpPieces[enemy][i]);
      if (!ptrPiece->mCaptured) {
         value -= ptrPiece->mValue +
                  ptrPiece->mpType->mWeights[enemy][ptrPiece->mSquare];
      }
   }

   return value;
}
