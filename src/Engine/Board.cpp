////////////////////////////////////////////////////////////////////////////
// Name:         Board.cpp
// Description:  Implementation of a class that represents a Board
// Created:      May 2, 2004 12:17:27 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "Board.hpp"
#include "Piece.hpp"
#include "PieceType.hpp"
#include "Move.hpp"
#include "RandomNumbers.hpp"

using std::vector;

// Constant declaration
const int Board::OFF_BOARD = -1;

//--------------------------------------------------------------------------
//       Class:  Board
//      Method:  Board
// Description:  Construct an instance of a Board object
//--------------------------------------------------------------------------
Board::Board(int width, int height, int zonerow, int piecetypes)
{
   mHeight = height;
   mWidth = width;
   mSize = mHeight * mWidth;
   mPieceTypes = piecetypes;

   mHashKey = (uint64)0;

   int size = 2*mSize*mPieceTypes;
   mpKeyComponents = new uint64 [2*mSize*mPieceTypes];

   for (int i=0; i < size; i++) {
      mpKeyComponents[i] = Random64();
   }

   mSquares.resize(mSize);
   
   for (int i = 0; i < 2; i++) {
     mColoredPieces[i].resize(mSize);
     mPromotionZones[i].resize(mSize);
     for (int j = 0; j < zonerow*mWidth; j++) {
        mPromotionZones[i].set(j + i*(mSize - zonerow*mWidth));
     }
   }
   
   mAllPieces.resize(mSize);
   mNoPieces.resize(mSize);
}

//--------------------------------------------------------------------------
//       Class:  Board
//      Method:  ~Board
// Description:  Destroys this instance of a Board
//--------------------------------------------------------------------------
Board::~Board()
{
   if (mpKeyComponents) {
      delete [] mpKeyComponents;
      mpKeyComponents = 0;
   }
}

//------------------------------------------------------------------------
//       Class:  Board
//      Method:  GetPieceInfo
// Description:  Return a vector of PieceInfo structures for each 
//               square on the board
//------------------------------------------------------------------------
void Board::GetPieceInfo(vector<PieceInfo>& squares)
{
   vector<PieceInfo>::size_type index;
   Piece* piecePtr;
   squares.resize(this->mSize);
   
   for (index = 0; index < squares.size(); index++) {
      piecePtr = this->mSquares[index];
      if (piecePtr) {
         if (piecePtr->mPromoted) {
            squares[index].mType = piecePtr->mpRelatedType->mTypeValue;
         }
         else {
            squares[index].mType = piecePtr->mTypeValue;
         }
         squares[index].mPromoted = piecePtr->mPromoted;
         squares[index].mSide = piecePtr->mColor;
        
      }
      else {
        squares[index].mType = -1;
        squares[index].mPromoted = false;
        squares[index].mSide = -1;
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  Board
//      Method:  MakeMove
// Description:  Make a move on the board
//--------------------------------------------------------------------------
uint64 Board::MakeMove(Move* move)
{
   uint64 oldkey = mHashKey;
   
   int color, square, type;
   Piece* mover = move->mpMover;
   color = 1 - mover->mColor;

   // Remove captured pieces from the board   
   for (unsigned int i = 0; i < move->mCaptures.size(); i++) {
      square = move->mCaptures[i]->mSquare;
      type = move->mCaptures[i]->mTypeValue;      

      // Update hash key
      mHashKey ^= KeyComponents(color, type, square);
      
      // Set captured flag to true
      move->mCaptures[i]->mCaptured = true;

      // Remove piece from board
      mSquares[square] = 0;
      
      // Update bitboards
      mColoredPieces[color].set(square, false);
      mAllPieces.set(square, false);
      mNoPieces.set(square, true);
   }
   
   // Relocate the moving piece
   color = 1 - color;
   
   // Remove piece from former position
   mSquares[move->mSource] = 0;
   
   mHashKey ^= KeyComponents(color, mover->mTypeValue, move->mSource);
   
   mColoredPieces[color].set(move->mSource, false);
   mAllPieces.set(move->mSource, false);
   mNoPieces.set(move->mSource, true);

   // Promote the moving piece if necessary
   if (move->mPromote) {
      PieceType* typePtr = mover->mpType;
      mover->mpType = mover->mpRelatedType;
      mover->mpRelatedType = typePtr;
      mover->mPromoted = true;
      mover->mTypeValue = mover->mpType->mTypeValue;
   }

   // Place the moved piece in its new position
   mSquares[move->mDestination] = mover;
   mover->mSquare = move->mDestination;   

   mHashKey ^= KeyComponents(color, mover->mTypeValue, move->mDestination);

   mColoredPieces[color].set(move->mDestination, true);
   mAllPieces.set(move->mDestination, true);
   mNoPieces.set(move->mDestination, false);
   
   return oldkey;
}

//--------------------------------------------------------------------------
//       Class:  Board
//      Method:  UnmakeMove
// Description:  Unmakes a move on the board
//--------------------------------------------------------------------------
uint64 Board::UnmakeMove(Move* move)
{
   int color, type, square;
   Piece* mover = move->mpMover;
   color = mover->mColor;
   
   // Remove the piece from its move destination square
   mSquares[move->mDestination] = 0;
   mHashKey ^= KeyComponents(color, mover->mTypeValue, move->mDestination);
   
   mColoredPieces[mover->mColor].set(move->mDestination, false);
   mAllPieces.set(move->mDestination, false);
   mNoPieces.set(move->mDestination, true);

   // Unpromote the moving piece if necessary
   if (move->mPromote) {
      PieceType* typePtr = mover->mpType;
      mover->mpType = mover->mpRelatedType;
      mover->mpRelatedType = typePtr;
      mover->mPromoted = false;
      mover->mTypeValue = mover->mpType->mTypeValue;
   }
   
   // Place the moving piece back in its move source square
   mSquares[move->mSource] = mover;
   mover->mSquare = move->mSource;
   
   mHashKey ^= KeyComponents(color, mover->mTypeValue, move->mSource);
   
   mColoredPieces[mover->mColor].set(move->mSource, true);
   mAllPieces.set(move->mSource, true);
   mNoPieces.set(move->mSource, false);

   // Add captured pieces back to the board
   color = 1 - color;
   for (unsigned int i = 0; i < move->mCaptures.size(); i++) {
      square = move->mCaptures[i]->mSquare;
      type = move->mCaptures[i]->mTypeValue;
      
      // Update hash key
      mHashKey ^= KeyComponents(color, type, square);
      
      // Unset captured flag
      move->mCaptures[i]->mCaptured = false;

      // Readd piece to board
      mSquares[square] = move->mCaptures[i];

      // Update bitboards
      mColoredPieces[color].set(square, true);
      mAllPieces.set(square, true);
      mNoPieces.set(square, false);
   }

   return mHashKey;
}

//--------------------------------------------------------------------------
//       Class:  Board
//      Method:  ResyncBitboards
// Description:  Resyncs the bitboards with the current board state
//--------------------------------------------------------------------------
void Board::ResyncBitboards()
{
   mAllPieces.reset();
   mColoredPieces[0].reset();
   mColoredPieces[1].reset();
   mNoPieces.reset();
   
   for (int i = 0; i < mSize; i++) {
      if (mSquares[i]) {
         mAllPieces.set(i);
         mColoredPieces[mSquares[i]->mColor].set(i);
      }
      else {
         mNoPieces.set(i);
      }
   }
}
