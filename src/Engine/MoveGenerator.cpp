////////////////////////////////////////////////////////////////////////////////
// Name:         MoveGenerator.cpp
// Description:  Implementation for class that generates moves for 
//               a board position
// Created:      Apr 20, 2004 11:43:23 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// =========================================================================
// Header file includes
// =========================================================================

// STL header files

// Local header files
#include "common.hpp"
#include "Game.hpp"
#include "Board.hpp"
#include "MoveGenerator.hpp"
#include "Move.hpp"
#include "Piece.hpp"
#include "ByValue.hpp"

using std::vector;

//--------------------------------------------------------------------------
//       Class:  MoveGenerator
//      Method:  MoveGenerator
// Description:  Construct an instance of a MoveGenerator object
//--------------------------------------------------------------------------
MoveGenerator::MoveGenerator(Game* game, int piececount)
{
   mpGame = game;
   mpPieces = game->mPieces;
   mPieceCount = piececount;
   mpPromotionZones = game->mpBoard->mPromotionZones;
}

//--------------------------------------------------------------------------
//       Class:  MoveGenerator
//      Method:  ~MoveGenerator
// Description:  Destroys this instance of a MoveGenerator
//--------------------------------------------------------------------------
MoveGenerator::~MoveGenerator()
{
}

//-------------------------------------------------------------------------
//       Class:  MoveGenerator
//      Method:  GenerateMoves
// Description:  Generate all possible moves from the current position
//-------------------------------------------------------------------------
void MoveGenerator::GenerateMoves(vector<Move>& movelist, int side)
{
   int i;
   Piece* piecePtr;
   vector<Move> droplist;

   // Generate all captures
   for (i = 0; i < mPieceCount; i++) {
      piecePtr = &(mpPieces[side][i]);
      if (!piecePtr->mCaptured)
         piecePtr->GenerateCaptures(movelist);
   }

   // Generate all non-captures
   for (i = 0; i < mPieceCount; i++) {
      piecePtr = &(mpPieces[side][i]);
      if (!piecePtr->mCaptured)
         piecePtr->GenerateNonCaptures(movelist);
   }
   
   // Test for promotions
   CheckForPromotions(movelist, side);

   // Allow any game specific processing
   mpGame->PostProcess(movelist, side);
}


//-------------------------------------------------------------------------
//       Class:  MoveGenerator
//      Method:  GetNextMove
// Description:  Return or generate the next move from the current
//               position of the board
//-------------------------------------------------------------------------
Move* MoveGenerator::GetNextMove(vector<Move>& movelist, int& phase, int side,
                                 Move* hashmove)
{
   int i; 
   Piece* piecePtr;
   Move*  movePtr;

   switch (phase) {
   // Process the move found in the transposition table if any
   case PROCESS_HASH_MOVE:
      phase = GENERATE_CAPTURES;
      if (hashmove) {
         movelist.push_back( *hashmove );
         return &(movelist.back());
      }
   // Generate captures  
   case GENERATE_CAPTURES:
      for (i = 0; i < mPieceCount; i++) {
         piecePtr = &(mpPieces[side][i]);
         if (!piecePtr->mCaptured)
            piecePtr->GenerateCaptures(movelist);
      }
      
      // Test for promotions
      CheckForPromotions(movelist, side);

      // Calculate sort values for the moves
      CalculateSortValues(movelist);
   
      // Allow any game specific processing
      mpGame->PostProcess(movelist, side);
      
      sort( movelist.begin(), movelist.end(), ByValue<Move>() );
      phase = RETURN_CAPTURES;
      
   // Return the captures that we have generated
   case RETURN_CAPTURES:
      while (true) {
         if ( movelist.empty() ) break;
         movePtr = &(movelist.back());
         if (!movePtr->mValidate || mpGame->IsValid(movePtr, side)) {
            return movePtr;
         }
         movelist.pop_back();
      }
      phase = GENERATE_NONCAPTURES;
   
   // Generate the noncapture moves
   case GENERATE_NONCAPTURES:
      for (i = 0; i < mPieceCount; i++) {
         piecePtr = &(mpPieces[side][i]);
         if (!piecePtr->mCaptured)
            piecePtr->GenerateNonCaptures(movelist);
      }
      
      // Test for promotions
      CheckForPromotions(movelist, side);
      
      // Allow any game specific processing
      mpGame->PostProcess(movelist, side);
      
      //sort( movelist.begin(), movelist.end(), ByHistory() );
      phase = RETURN_NONCAPTURES;
   
   // Return the noncapture moves that we have generated
   case RETURN_NONCAPTURES:
      while (true) {
         if ( movelist.empty() ) break;
         movePtr = &(movelist.back());
         if (!movePtr->mValidate || mpGame->IsValid(movePtr, side)) {
            return movePtr;
         }
         movelist.pop_back();
      }
   }

   // No more moves left
   return 0;   
}

//-------------------------------------------------------------------------
//       Class:  MoveGenerator
//      Method:  GetNextCapture
// Description:  Return or generate the next capture from the current
//               position of the board
//-------------------------------------------------------------------------
Move* MoveGenerator::GetNextCapture(vector<Move>& movelist, int& phase, int side)
{
   int i; 
   Piece* piecePtr;

   switch (phase) {
   // Generate captures  
   case GENERATE_CAPTURES:
      for (i = 0; i < mPieceCount; i++) {
         piecePtr = &(mpPieces[side][i]);
         if (!piecePtr->mCaptured)
            piecePtr->GenerateCaptures(movelist);
      }
      
      // Test for promotions
      CheckForPromotions(movelist, side);
      
      // Allow any game specific processing
      mpGame->PostProcess(movelist, side);
      
      // Calculate sort values for the moves
      CalculateSortValues(movelist);
      
      sort( movelist.begin(), movelist.end(), ByValue<Move>() );
      phase = RETURN_CAPTURES;
      
   // Return the captures that we have generated
   case RETURN_CAPTURES:
      if ( !movelist.empty() ) {
         return &(movelist.back());
      }
   }

   // No more captures left
   return 0;   
}

//--------------------------------------------------------------------------
//       Class:  MoveGenerator
//      Method:  CheckForPromotions
// Description:  Checks movelist for possible promoting moves
//--------------------------------------------------------------------------
void MoveGenerator::CheckForPromotions(vector<Move>& movelist, int side)
{
   vector<Move> promotions;
   
   for (unsigned int i = 0; i < movelist.size(); i++) {
      // Force promotion?
      if (mpGame->ForcePromotion(movelist[i])) {
         movelist[i].mPromote = true;
      }
      else {
         Move promotion(movelist[i]);
         if ((mpPromotionZones[side].test(promotion.mDestination) ||
              mpPromotionZones[side].test(promotion.mSource) ||
              ((promotion.mVisited != -1) &&
               (mpPromotionZones[side].test(promotion.mVisited)))) &&
             !(promotion.mpMover->mPromoted) &&
             promotion.mpMover->mpRelatedType) {
            promotion.mPromote = true;
            promotions.push_back(promotion);
         }
      }
   }
   
   if (!promotions.empty())
      movelist.insert(movelist.end(), promotions.begin(), promotions.end());
}

//--------------------------------------------------------------------------
//       Class:  MoveGenerator
//      Method:  CalculateSortValues
// Description:  Calculates a sorting values for the moves in a list by:
//               (a) the value of the captured pieces - the value of the
//                   moving piece if this expression is > 0; or,
//               (b) the value returned by analyzing the pieces attacking
//                   the destination square to see which side comes out
//                   favorably
//--------------------------------------------------------------------------
void MoveGenerator::CalculateSortValues(vector<Move>& movelist)
{
   int value, delta, basecolor, color, square;
   unsigned int j;
   Piece *moverPtr, *piecePtr;
   vector<Piece*> attackers[2], nonattackers[2];
   
   for (unsigned int i = 0; i < movelist.size(); i++) {
      moverPtr = movelist[i].mpMover;
      value = 0;
      // Sum the values of the captured pieces
      for (j=0; j < movelist[i].mCaptures.size(); j++) {
         value += movelist[i].mCaptures[j]->mValue;
      }

      if (moverPtr->mValue < value) {
         // If the sum of the captures is greater than the moving piece
         // in material/positional value set the move's value to this
         movelist[i].mValue = value - moverPtr->mValue;
      }
      else {
         square = movelist[i].mDestination;
         // Separate the available pieces into threatening pieces
         // and non-threatening pieces, excluding the moving piece
         for (color = 0; color < 2; color++) {
            attackers[color].clear();
            nonattackers[color].clear();
            for (j = 0; j < mPieceCount; j++) {
               piecePtr = &(mpPieces[color][j]);
               if ((piecePtr != moverPtr) && (!piecePtr->mCaptured)) {
                  if (piecePtr->mpType->ThreatensSquare(piecePtr, square))
                     attackers[color].push_back(piecePtr);
                  else
                     nonattackers[color].push_back(piecePtr);
               }
            }
         }

         // Sort the attacking pieces by least value
         sort(attackers[0].begin(), attackers[0].end(), ByValue<Piece*,true>());
         sort(attackers[1].begin(), attackers[1].end(), ByValue<Piece*,true>());

         // Push the moving piece to the back of the friendly attackers
         attackers[moverPtr->mColor].push_back(moverPtr);

         // Alternate between friendly and enemy pieces
         // "capturing" until achieving a final material 
         // score for the friendly side
         color = moverPtr->mColor;
         basecolor = moverPtr->mColor;
         delta = 0;
         do {
            value += delta;
            if (color == basecolor) {
               delta = -1*attackers[color].back()->mValue;
            }
            else {
               delta = attackers[color].back()->mValue;
            }
            attackers[color].pop_back();
            color = 1 - color;
         } while (!attackers[color].empty());
         
         movelist[i].mValue = value;
      }
   }
}
