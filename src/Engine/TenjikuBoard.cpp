////////////////////////////////////////////////////////////////////////////
// Name:         TenjikuBoard.cpp
// Description:  Implementation for a class that represents a 
//               tenjiku shogi board
// Created:      09/10/2004 07:14:53 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 22:17:32 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// mShogi header files
#include "TenjikuBoard.hpp"
#include "Piece.hpp"
#include "PieceType.hpp"
#include "Move.hpp"
#include "RandomNumbers.hpp"

using std::vector;

//--------------------------------------------------------------------------
//       Class:  TenjikuBoard
//      Method:  TenjikuBoard
// Description:  Construct an instance of a TenjikuBoard object
//--------------------------------------------------------------------------
TenjikuBoard::TenjikuBoard(int width, int height, int zonerow, int piecetypes)
: Board(width, height, zonerow, height)
{
   mpBurnBitboards = 0;
}

//--------------------------------------------------------------------------
//       Class:  TenjikuBoard
//      Method:  MakeMove
// Description:  Make a move on the board
//--------------------------------------------------------------------------
uint64 TenjikuBoard::MakeMove(Move* move)
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
  
   // Switch back to our moving piece's color 
   color = 1 - color;
   
   // Remove piece from former position
   mSquares[move->mSource] = 0;
   
   mHashKey ^= KeyComponents(color, mover->mTypeValue, move->mSource);
   
   mColoredPieces[color].set(move->mSource, false);
   mAllPieces.set(move->mSource, false);
   mNoPieces.set(move->mSource, true);

   // Check whether this is a suicide move
   if (move->mSuicide) {
      // Set captured flag to true
      mover->mCaptured = true;
   }   
   else {
      // Promote the moving piece if necessary
      if (move->mPromote) {
         PieceType* typePtr = mover->mpType;
         mover->mpType = mover->mpRelatedType;
         mover->mpRelatedType = typePtr;
         mover->mPromoted = true;
         mover->mTypeValue = mover->mpType->mTypeValue;
   
         // Check whether the moving piece has promoted to a Fire Demon
         if (mover->mTypeValue == mFireDemonValue) {
            // Add this fire demon to our list
            mpFireDemons[color].push_back(mover);
         }
      }

      // Place the moved piece in its new position
      mSquares[move->mDestination] = mover;
      mover->mSquare = move->mDestination;   

      mHashKey ^= KeyComponents(color, mover->mTypeValue, move->mDestination);
   
      mColoredPieces[color].set(move->mDestination, true);
      mAllPieces.set(move->mDestination, true);
      mNoPieces.set(move->mDestination, false);
   }

   // Update the burn zones
   UpdateBurnZones();

#ifdef VERIFY
   for (square = 0; square < mSquares.size(); square++) {
      if (mSquares[square]) {
         if (mSquares[square]->mColor == 0) {
            assert(mColoredPieces[0].test(square) &&
                   !mColoredPieces[1].test(square));
         }
         else {
            assert(mColoredPieces[1].test(square) &&
                   !mColoredPieces[0].test(square));
         }
      }
      else {
         assert(!(mColoredPieces[0].test(square) ||
                  mColoredPieces[1].test(square)));
      }
   }
#endif

   return oldkey;
}

//--------------------------------------------------------------------------
//       Class:  TenjikuBoard
//      Method:  UnmakeMove
// Description:  Unmakes a move on the board
//--------------------------------------------------------------------------
uint64 TenjikuBoard::UnmakeMove(Move* move)
{
   int color, type, square;
   Piece* mover = move->mpMover;
   color = mover->mColor;
   
   // Uncapture a piece if this was a suicide move
   if (move->mSuicide) {
      mover->mCaptured = false;
   }
   else {
      // Remove the piece from its move destination square
      mSquares[move->mDestination] = 0;
      mHashKey ^= KeyComponents(color, mover->mTypeValue, move->mDestination);
      
      mColoredPieces[mover->mColor].set(move->mDestination, false);
      mAllPieces.set(move->mDestination, false);
      mNoPieces.set(move->mDestination, true);

      // Unpromote the moving piece if necessary
      if (move->mPromote) {
         // Check whether the moving piece was promoted to a Fire Demon
         if (mover->mTypeValue == mFireDemonValue) {
            // Remove this piece from our fire demon list
            mpFireDemons[color].pop_back();
         }

         PieceType* typePtr = mover->mpType;
         mover->mpType = mover->mpRelatedType;
         mover->mpRelatedType = typePtr;
         mover->mPromoted = false;
         mover->mTypeValue = mover->mpType->mTypeValue;
      }
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

   // Update the burn zones
   UpdateBurnZones();

#ifdef VERIFY
   for (square = 0; square < mSquares.size(); square++) {
      if (mSquares[square]) {
         if (mSquares[square]->mColor == 0) {
            assert(mColoredPieces[0].test(square) &&
                   !mColoredPieces[1].test(square));
         }
         else {
            assert(mColoredPieces[1].test(square) &&
                   !mColoredPieces[0].test(square));
         }
      }
      else {
         assert(!(mColoredPieces[0].test(square) ||
                  mColoredPieces[1].test(square)));
      }
   }
#endif

   return mHashKey;
}

//--------------------------------------------------------------------------
//       Class:  TenjikuBoard
//      Method:  UpdateBurnZones
// Description:  Updates the bitboard patterns that represent the current
//               squares that are adjacent to enemy fire demons
//--------------------------------------------------------------------------
void TenjikuBoard::UpdateBurnZones() 
{
   vector<Piece*>::size_type index, size;
   Piece* piecePtr;
   int enemy;
   for (int color = 0; color < 2; color++) {
      enemy = 1 - color;
      size = mpFireDemons[enemy].size();
      mBurnZones[color].reset();
      for (index = 0; index < size; index++) {
         piecePtr = mpFireDemons[enemy][index];
         if (!piecePtr->mCaptured) {
            mBurnZones[color] |= mpBurnBitboards[piecePtr->mSquare];
         }
      }
   }
}


