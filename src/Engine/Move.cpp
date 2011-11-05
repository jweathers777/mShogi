// Name:         Move.cpp
// Description:  Implementation for a class that represents a move
// Created:      Apr 20, 2004 2:12:13 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// STL header files
#include <vector>

// mShogi header files
#include "common.hpp"
#include "Board.hpp"
#include "Move.hpp"
#include "MoveRec.hpp"
#include "Piece.hpp"

using std::vector;

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  Move
// Description:  Constructs an instance of a move
//--------------------------------------------------------------------------
Move::Move(Piece* mover, int source, int destination, Piece* capture,
           bool validate)
{
   mSource = source;
   mVisited = Board::OFF_BOARD;
   mDestination = destination;
   
   int color = mover->mColor;
   mValue = mover->mpType->mWeights[color][destination] -
            mover->mpType->mWeights[color][source];
   
   mpMover = mover;
   
   mValidate = validate;
   mPromote = false;
   mSuicide = false;
   
   if (capture) {
     mCaptures.push_back(capture);
     int square = capture->mSquare;
     mValue += capture->mValue + capture->mpType->mWeights[1-color][square];
   }
}

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  Move
// Description:  Constructs an instance of a move
//--------------------------------------------------------------------------
Move::Move(Piece* mover, int source, int destination, 
           vector<Piece*>& captures, bool validate) : mCaptures(captures)
{
   mSource = source;
   mVisited = Board::OFF_BOARD;
   mDestination = destination;
   
   int color = mover->mColor;
   mValue = mover->mpType->mWeights[color][destination] -
            mover->mpType->mWeights[color][source];
   
   mpMover = mover;
   
   mValidate = validate;
   mPromote = false;
   mSuicide = false;
}

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  Move
// Description:  Constructs an instance of a move
//--------------------------------------------------------------------------
Move::Move(Piece* mover, int source, int visited, int destination, 
           Piece* capture, bool validate)
{
   mSource = source;
   mVisited = visited;
   mDestination = destination;
   
   int color = mover->mColor;
   mValue = mover->mpType->mWeights[color][destination] -
            mover->mpType->mWeights[color][source];

   mpMover = mover;
   
   mValidate = validate;
   mPromote = false;
   mSuicide = false;
   
   if (capture) {
     mCaptures.push_back(capture);
     int square = capture->mSquare;
     mValue += capture->mValue + capture->mpType->mWeights[1-color][square];
   }
}

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  Move
// Description:  Constructs an instance of a move
//--------------------------------------------------------------------------
Move::Move(Piece* mover, int source, int visited, int destination, 
           vector<Piece*>& captures, bool validate) : mCaptures(captures)
{
   mSource = source;
   mVisited = visited;
   mDestination = destination;
   
   int color = mover->mColor;
   mValue = mover->mpType->mWeights[color][destination] -
            mover->mpType->mWeights[color][source];

   mpMover = mover;
   
   mValidate = validate;
   mPromote = false;
   mSuicide = false;
}

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  Move
// Description:  Constructs an instance of a move from another move
//--------------------------------------------------------------------------
Move::Move(const Move& move) : mCaptures(move.mCaptures)
{
   mSource = move.mSource;
   mVisited = move.mVisited;
   mDestination = move.mDestination;
   
   mValue = move.mValue;
   
   mpMover = move.mpMover;
   
   mValidate = move.mValidate;
   mPromote = move.mPromote;
   mSuicide = move.mSuicide;
}

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  Copy
// Description:  Copies the contents of a move into this move
//--------------------------------------------------------------------------
void Move::Copy(const Move& move)
{
   mSource = move.mSource;
   mVisited = move.mVisited;
   mDestination = move.mDestination;
   
   mValue = move.mValue;

   mpMover = move.mpMover;
  
   mValidate = move.mValidate; 
   mPromote = move.mPromote;
   mSuicide = move.mSuicide;
   
   mCaptures = move.mCaptures;
}

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  operator==
// Description:  Returns true if two moves are equal
//--------------------------------------------------------------------------
bool Move::operator== (const Move& move) const
{
   return (mSource == move.mSource) &&
          (mVisited == move.mVisited) &&
          (mDestination == move.mDestination) &&
          (mpMover == move.mpMover) &&
          (mPromote == move.mPromote) &&
          (mSuicide == move.mSuicide);
}

//--------------------------------------------------------------------------
//       Class:  Move
//      Method:  operator==
// Description:  Returns true if the move equals the given move record
//--------------------------------------------------------------------------
bool Move::operator== (const MoveRec& moverec) const
{
   bool promote = moverec.mFlags & 1;
   bool suicide = moverec.mFlags & 2;
   
   return (mSource == moverec.mSource) &&
          (mVisited == moverec.mVisited) &&
          (mDestination == moverec.mDestination) &&
          (mpMover == (Piece*)(moverec.mPiecePtr)) &&
          (mPromote == promote) &&
          (mSuicide == suicide);
}


//------------------------------------------------------------------------
//       Class:  Move
//      Method:  FillMoveRec
// Description:  Fill a move record with data from this move
//------------------------------------------------------------------------
void Move::FillMoveRec(MoveRec& moverec)
{
   moverec.mSource = mSource;
   moverec.mVisited = mVisited;
   moverec.mDestination = mDestination;
   moverec.mPiecePtr = (void *)mpMover;
   moverec.mFlags = 0;
   if (mPromote) {
      moverec.mFlags |= 1;
   }
   if (mSuicide) {
      moverec.mFlags |= 2;
   }
}
