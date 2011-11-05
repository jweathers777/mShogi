////////////////////////////////////////////////////////////////////////////
// Name:         ChuShogi.cpp
// Description:  Interface for a class that represents the 
//               game of Chu Shogi
// Created:      02/04/2005 11:10:32 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2005 John Weathers
////////////////////////////////////////////////////////////////////////////

// STL declaration
#include <vector>

// mShogi header files
#include "common.hpp"
#include "ChuShogi.hpp"
#include "Board.hpp"
#include "MoveRec.hpp"
#include "MoveGenerator.hpp"
#include "Evaluator.hpp"
#include "Notator.hpp"

// Chu Shogi Pieces header files
#include "Pawn.hpp"
#include "GoBetween.hpp"
#include "CopperGeneral.hpp"
#include "SilverGeneral.hpp"
#include "FerociousLeopard.hpp"
#include "BlindTiger.hpp"
#include "DrunkElephant.hpp"
#include "GoldGeneral.hpp"
#include "Lance.hpp"
#include "ReverseChariot.hpp"
#include "Kylin.hpp"
#include "Phoenix.hpp"
#include "SideMover.hpp"
#include "VerticalMover.hpp"
#include "Bishop.hpp"
#include "Rook.hpp"
#include "DragonHorse.hpp"
#include "DragonKing.hpp"
#include "FreeKing.hpp"
#include "Lion.hpp"
#include "King.hpp"
#include "Jewel.hpp"
#include "Tokin.hpp"
#include "FlyingStag.hpp"
#include "CrownPrince.hpp"
#include "Whale.hpp"
#include "WhiteHorse.hpp"
#include "FreeBoar.hpp"
#include "FlyingOx.hpp"
#include "HornedFalcon.hpp"
#include "SoaringEagle.hpp"

// Define useful shorthand for describing initial board lay out
#define bK   ChuShogi::JEWEL + 1
#define bLn  ChuShogi::LION + 1
#define bFK  ChuShogi::FREE_KING + 1
#define bDK  ChuShogi::DRAGON_KING + 1
#define bDH  ChuShogi::DRAGON_HORSE + 1
#define bR   ChuShogi::ROOK + 1
#define bB   ChuShogi::BISHOP + 1
#define bVM  ChuShogi::VERTICAL_MOVER + 1
#define bSM  ChuShogi::SIDE_MOVER + 1
#define bPh  ChuShogi::PHOENIX + 1
#define bKy  ChuShogi::KYLIN + 1
#define bRC  ChuShogi::REVERSE_CHARIOT + 1
#define bL   ChuShogi::LANCE + 1
#define bG   ChuShogi::GOLD_GENERAL + 1
#define bDE  ChuShogi::DRUNK_ELEPHANT + 1
#define bBT  ChuShogi::BLIND_TIGER + 1
#define bFL  ChuShogi::FEROCIOUS_LEOPARD + 1
#define bS   ChuShogi::SILVER_GENERAL + 1
#define bC   ChuShogi::COPPER_GENERAL + 1
#define bGB  ChuShogi::GO_BETWEEN + 1
#define bP   ChuShogi::PAWN + 1

#define wK   ChuShogi::KING  + 23
#define wLn  ChuShogi::LION  + 23
#define wFK  ChuShogi::FREE_KING  + 23
#define wDK  ChuShogi::DRAGON_KING  + 23
#define wDH  ChuShogi::DRAGON_HORSE  + 23
#define wR   ChuShogi::ROOK  + 23
#define wB   ChuShogi::BISHOP  + 23
#define wVM  ChuShogi::VERTICAL_MOVER  + 23
#define wSM  ChuShogi::SIDE_MOVER  + 23
#define wPh  ChuShogi::PHOENIX  + 23
#define wKy  ChuShogi::KYLIN  + 23
#define wRC  ChuShogi::REVERSE_CHARIOT  + 23
#define wL   ChuShogi::LANCE  + 23
#define wG   ChuShogi::GOLD_GENERAL  + 23
#define wDE  ChuShogi::DRUNK_ELEPHANT  + 23
#define wBT  ChuShogi::BLIND_TIGER  + 23
#define wFL  ChuShogi::FEROCIOUS_LEOPARD  + 23
#define wS   ChuShogi::SILVER_GENERAL  + 23
#define wC   ChuShogi::COPPER_GENERAL  + 23
#define wGB  ChuShogi::GO_BETWEEN  + 23
#define wP   ChuShogi::PAWN  + 23

using std::vector;

// Constants
const int ChuShogi::WIDTH;
const int ChuShogi::HEIGHT;
const int ChuShogi::SIZE;
const int ChuShogi::PIECE_COUNT;
const int ChuShogi::PIECE_TYPES;

// Array that defines the inital board lay out
const int ChuShogi::mStartingPattern[] = {
   wL,  wFL, wC,  wS,  wG,  wK,  wDE, wG,  wS,  wC,  wFL, wL,
   wRC, 0,   wB,  0,   wBT, wKy, wPh, wBT, 0,   wB,  0,   wRC,
   wSM, wVM, wR,  wDH, wDK, wLn, wFK, wDK, wDH, wR,  wVM, wSM,
   wP,  wP,  wP,  wP,  wP,  wP,  wP,  wP,  wP,  wP,  wP,  wP,
   0,   0,   0,   wGB, 0,   0,   0,   0,   wGB, 0,   0,   0,
   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
   0,   0,   0,   bGB, 0,   0,   0,   0,   bGB, 0,   0,   0,
   bP,  bP,  bP,  bP,  bP,  bP,  bP,  bP,  bP,  bP,  bP,  bP,
   bSM, bVM, bR,  bDH, bDK, bFK, bLn, bDK, bDH, bR,  bVM, bSM,
   bRC, 0,   bB,  0,   bBT, bPh, bKy, bBT, 0,   bB,  0,   bRC,
   bL,  bFL, bC,  bS,  bG,  bDE, bK,  bG,  bS,  bC,  bFL, bL
};

// Array that defines promotions
const int ChuShogi::mPromotionMap[] = {
  // PAWN,  GO_BETWEEN,     
     TOKIN, DRUNK_ELEPHANT, 
  // COPPER_GENERAL, SILVER_GENERAL, FEROCIOUS_LEOPARD, BLIND_TIGER, 
     SIDE_MOVER,     VERTICAL_MOVER, BISHOP,            FLYING_STAG, 
  // DRUNK_ELEPHANT, GOLD_GENERAL, LANCE,       REVERSE_CHARIOT,
     CROWN_PRINCE,   ROOK,         WHITE_HORSE, WHALE,
  // KYLIN, PHOENIX,   SIDE_MOVER, VERTICAL_MOVER,
     LION,  FREE_KING, FREE_BOAR,  FLYING_OX,
  // BISHOP,       ROOK,        DRAGON_HORSE,  DRAGON_KING,
     DRAGON_HORSE, DRAGON_KING, HORNED_FALCON, SOARING_EAGLE,
  // FREE_KING,    LION,
     NO_PROMOTION, NO_PROMOTION,
  // KING,         JEWEL
     NO_PROMOTION, NO_PROMOTION
};

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  ChuShogi
// Description:  Create an instance of a chu shogi game
//--------------------------------------------------------------------------
ChuShogi::ChuShogi()
{
   // Create a 12x12 board
   mpBoard = new Board(WIDTH, HEIGHT, 4, PIECE_TYPES);

   // Create the piece types that we need
   mPieceTypes.push_back( new Pawn(mpBoard, 1000, PAWN) );
   mPieceTypes.push_back( new GoBetween(mpBoard, 1200, GO_BETWEEN) );
   mPieceTypes.push_back( new CopperGeneral(mpBoard, 2534, COPPER_GENERAL) );
   mPieceTypes.push_back( new SilverGeneral(mpBoard, 3310, SILVER_GENERAL) );
   mPieceTypes.push_back( new FerociousLeopard(mpBoard, 3425, FEROCIOUS_LEOPARD) );
   mPieceTypes.push_back( new BlindTiger(mpBoard, 3580, BLIND_TIGER) );
   mPieceTypes.push_back( new DrunkElephant(mpBoard, 4230, DRUNK_ELEPHANT) );
   mPieceTypes.push_back( new GoldGeneral(mpBoard, 4355, GOLD_GENERAL) );
   mPieceTypes.push_back( new Lance(mpBoard, 4563, LANCE) );
   mPieceTypes.push_back( new ReverseChariot(mpBoard, 4721, REVERSE_CHARIOT) );
   mPieceTypes.push_back( new Kylin(mpBoard, 5211, KYLIN) );
   mPieceTypes.push_back( new Phoenix(mpBoard, 5291, PHOENIX) );
   mPieceTypes.push_back( new SideMover(mpBoard, 5371, SIDE_MOVER) );
   mPieceTypes.push_back( new VerticalMover(mpBoard, 5620, VERTICAL_MOVER) );
   mPieceTypes.push_back( new Bishop(mpBoard, 8644, BISHOP) );
   mPieceTypes.push_back( new Rook(mpBoard, 8776, ROOK) );
   mPieceTypes.push_back( new DragonHorse(mpBoard, 11456, DRAGON_HORSE) );
   mPieceTypes.push_back( new DragonKing(mpBoard, 11631, DRAGON_KING) );
   mPieceTypes.push_back( new FreeKing(mpBoard, 13800, FREE_KING) );
   mPieceTypes.push_back( new Lion(mpBoard, 14578, LION) );
   mPieceTypes.push_back( new King(mpBoard, 650000, KING) );
   mPieceTypes.push_back( new Jewel(mpBoard, 650000, JEWEL) );
  
   mPieceTypes.push_back( new Tokin(mpBoard, 1501, TOKIN) );
   mPieceTypes.push_back( new FlyingStag(mpBoard, 4810, FLYING_STAG) );
   mPieceTypes.push_back( new CrownPrince(mpBoard, 5135, CROWN_PRINCE) );
   mPieceTypes.push_back( new Whale(mpBoard, 5240, WHALE) );
   mPieceTypes.push_back( new WhiteHorse(mpBoard, 5276, WHITE_HORSE) );
   mPieceTypes.push_back( new FreeBoar(mpBoard, 9234, FREE_BOAR) );
   mPieceTypes.push_back( new FlyingOx(mpBoard, 9375, FLYING_OX) );
   mPieceTypes.push_back( new HornedFalcon(mpBoard, 12451, HORNED_FALCON) );
   mPieceTypes.push_back( new SoaringEagle(mpBoard, 12672, SOARING_EAGLE) );
   
   // Assign positional weights to the piece types
   SetWeights();
   
   // Create pieces and place them on the board
   Reset();
   
   // Create a new move generator
   mpMoveGenerator = new MoveGenerator(this, PIECE_COUNT);
   
   // Create a new evaluator
   mpEvaluator = new Evaluator(mPieces);
   
   // Create a new Notator
   mpNotator = new Notator(mpBoard);

   // Turn on the special lion capture rules for Chu Shogi
   Lion::sLionCaptureRules = true;
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  ~ChuShogi
// Description:  Destroy this instance of a mini shogi game
//--------------------------------------------------------------------------
ChuShogi::~ChuShogi()
{
   for (unsigned int i = 0; i < mPieceTypes.size(); i++) {
      if (mPieceTypes[i]) {
         delete mPieceTypes[i];
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  IsLegal
// Description:  Returns true if the given move for the given side is legal
//--------------------------------------------------------------------------
bool ChuShogi::IsLegal(Move* move, int side)
{
   unsigned int i;
   Piece* mover = move->mpMover;
   
   // Is the right side that is moving?
   if (mover->mColor != side) {
      return false;
   }
   
   // Validate the captures
   for (i=0; i < move->mCaptures.size(); i++) {
      if (move->mCaptures[i]->mColor == side) {  
         return false; // Cannot capture own piece
      }
   }

   // Is this a forced promotion?
   if (ForcePromotion(*move)) {
       move->mPromote = true;
   }

   // Can this piece actually make this move?
   vector<Move> movelist;
   mover->mpType->GenerateCaptures(movelist, mover);   
   mover->mpType->GenerateNonCaptures(movelist, mover);
   mpMoveGenerator->CheckForPromotions(movelist, side);
   PostProcess(movelist, side);

   for (i=0; i < movelist.size(); i++) {
      if (movelist[i] == *move) {
         return true;
      }
   }

   // No matching legal move was found
   return false;   
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  IsValid
// Description:  Returns true if the given pseudo-legal move for the 
//               given side is truly legal
//--------------------------------------------------------------------------
bool ChuShogi::IsValid(Move* move, int side)
{
   int value;
   
   // Is this a double capture move?
   if (move->mCaptures.size() == 2) {
      // Is the first capture neither a pawn nor a go-between?
      value = move->mCaptures[0]->mTypeValue;
      if ((value != GO_BETWEEN) && (value != PAWN)) {
         return true;
      }
   }
      
   // Temporarily remove your attacking lion from the board
   int square = move->mSource;
   
   Piece* attackerLion = mpBoard->mSquares[square];
   mpBoard->mSquares[square] = 0;
   mpBoard->mColoredPieces[side].set(square, false);
   mpBoard->mAllPieces.set(square, false);
   mpBoard->mNoPieces.set(square, true);

   // Is the enemy lion that is being captured protected when you remove
   // your attacking lion from the board?
   bool result = true;
   int color = 1-side;
   Piece* piecePtr;
   
   square = move->mDestination;
   for (int i = 0; i < PIECE_COUNT; i++) {
      piecePtr = &(mPieces[color][i]);
      if (piecePtr->mpType->ThreatensSquare(piecePtr, square)) {
         result = false;
         break;
      }  
   }   

   // Restore your attacking lion to the board
   square = move->mSource;
   mpBoard->mSquares[square] = attackerLion;
   mpBoard->mColoredPieces[side].set(square, true);
   mpBoard->mAllPieces.set(square, true);
   mpBoard->mNoPieces.set(square, false);

   return result;
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  CanPromote
// Description:  Returns true if the given move offers the 
//               opportunity for promotion 
//--------------------------------------------------------------------------
bool ChuShogi::CanPromote(Move* move)
{
   vector<Move> movelist;
   int side = move->mpMover->mColor;

   movelist.push_back(*move);
   mpMoveGenerator->CheckForPromotions(movelist, side);
   
   return (movelist.size() > 1);
}

//------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  CanMoveTwice
// Description:  Returns true if this move offers the opportunity to move
//               twice in one move
//------------------------------------------------------------------------
bool ChuShogi::CanMoveTwice(int start, int finish)
{
   int delta;
   int factor;
   bool result = false;
   
   if (mpBoard->mSquares[start]) {
      int factor = (mpBoard->mSquares[start]->mColor == BLACK) ? -1 : 1;
      switch (mpBoard->mSquares[start]->mpType->mTypeValue) {
      case LION:
         delta = (finish > start)? (finish - start):(start - finish);
         if ((delta == 1) || 
             (delta == WIDTH) ||
             (delta == (WIDTH+1)) || 
             (delta == (WIDTH-1)) ) 
         {
            result = true;
         }
         break;
      case SOARING_EAGLE:
         delta = factor*(finish - start);
         if ((delta == (WIDTH+1)) || 
             (delta == (WIDTH-1)) ) 
         {
            result = true;
         }
         break;
      case HORNED_FALCON:
         delta = factor*(finish - start);
         if (delta == WIDTH) {
            result = true;
         }
         break;
      }
   }

   return result;
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  GameWon
// Description:  Returns true if the current game state is a winning state
//--------------------------------------------------------------------------
bool ChuShogi::GameWon(int side)
{
   int enemy = 1-side;
   if (mpCrownPrinces[enemy])
      return (mpCrownPrinces[enemy]->mCaptured && 
              mpKings[enemy]->mCaptured);
   else
      return mpKings[enemy]->mCaptured;
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  GameDrawn
// Description:  Returns true if the current game state is drawn
//--------------------------------------------------------------------------
bool ChuShogi::GameDrawn()
{
   bool drawn = false;
   int reps = 0;
   uint64 key = mpBoard->mHashKey;
   
   // Check for draw by repetition
   for (unsigned int i=0; i < mHashKeys.size(); i++) {
      if (mHashKeys[i] == key) {
         // We have a repetition
         reps++;
      }

      if (reps >= 3) {
         drawn = true;
         break;
      }
   }

   //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
   // Warning: code below needs to be adjusted to account 
   // for crown princes at some point
   //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
   
   // Check for draw by naked kings
   if (!drawn) {
      int pieces = 0;
      for (int color = 0; color < 2; color++) {
         for (unsigned int i = 0; i < mPieces[color].size(); i++) {
            if (!mPieces[color][i].mCaptured) 
               pieces++;
         }
      }

      if (pieces == 2) {
         drawn = true;

         // There are only the two kings left on the board
         // Can one king capture the other?
         vector<Move> movelist;
         Piece* target = mpKings[0];
         Piece* mover = mpKings[1];
         mover->mpType->GenerateCaptures(movelist, mover);

         for (unsigned int i = 0; i < movelist.size(); i++) {
            if (movelist[i].mDestination == target->mSquare) {
               drawn = false;
               break;
            }
         }
      }
   }

   return drawn;
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  ForcePromotion
// Description:  Returns true if this move forces a promotion
//--------------------------------------------------------------------------
bool ChuShogi::ForcePromotion(const Move& move) 
{
   //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
   // Needs to be implemented
   //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
   return false;
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  SetWeights
// Description:  Assign positional weight matrices to the piece types
//--------------------------------------------------------------------------
void ChuShogi::SetWeights()
{
   for (unsigned int i=0; i < mPieceTypes.size(); i++) {
      mPieceTypes[i]->mWeights[0].resize(SIZE);
      mPieceTypes[i]->mWeights[1].resize(SIZE);
      for (int square = 0; square < SIZE; square++) {
         mPieceTypes[i]->mWeights[0][square] = 0;
         mPieceTypes[i]->mWeights[1][square] = 0;
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  Reset
// Description:  Resets the game to its initial state
//--------------------------------------------------------------------------
void ChuShogi::Reset() 
{
   unsigned int i;
   int piecetype, color, promoted;
   
   // Remove old pieces from the board and delete them
   for (color = 0; color < 2; color++) {
      mPieces[color].clear();
   }

   // Re-initialize pointers on board squares to the NULL pointer
   mpBoard->mSquares.assign(SIZE, (Piece*)0);

   // Re-initialize the board hash key to an empty board key
   mpBoard->mHashKey = (uint64)0;

   // Create a new set of pieces
   for (i = 0; i < SIZE; i++) {
      if (mStartingPattern[i] != 0) {
         piecetype = (mStartingPattern[i] - 1) % UNPROMOTED_PIECE_TYPES;
         color = (mStartingPattern[i] - 1) / UNPROMOTED_PIECE_TYPES;
         if (mPromotionMap[piecetype] != NO_PROMOTION) {
            promoted = mPromotionMap[piecetype];
            mPieces[color].push_back(
               Piece(mPieceTypes[piecetype], color, i, mPieceTypes[promoted])
               );
         }
         else {
            mPieces[color].push_back(Piece(mPieceTypes[piecetype], color, i));
         }
      }
   }

   int square;

   // At the start of the game there are no crown princes
   mpCrownPrinces[0] = mpCrownPrinces[1] = 0;

   // Place the new pieces on the board in their starting positions
   for (color = 0; color < 2; color++) {
      for (i = 0; i < mPieces[color].size(); i++) {
         square = mPieces[color][i].mSquare;
         piecetype = mPieces[color][i].mTypeValue;
         mpBoard->mSquares[square] =&(mPieces[color][i]);
         
         // Establish pointers to the kings for quick checks 
         // on whether a king has been captured
         if ((piecetype == KING) || (piecetype == JEWEL)) {
            mpKings[color] = mpBoard->mSquares[square];
         }
 
         // Update the hash key
         mpBoard->mHashKey ^= mpBoard->KeyComponents(color, piecetype, square);
      }
   }

   // Resync the boards bitboards
   mpBoard->ResyncBitboards();

   // Set the status to in progress
   mStatus = IN_PROGRESS;

   // Clear the hashkeys history
   mHashKeys.clear();
}

//--------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  CreateMove
// Description:  Returns a pointer to a new move constructed 
//               from a move record
//--------------------------------------------------------------------------
Move* ChuShogi::CreateMove(const MoveRec& moverec)
{
   unsigned int i;
   Piece* mover = (Piece*)moverec.mPiecePtr;
   Move* movePtr = 0;
   
   // Generate the possible moves that this piece can make
   vector<Move> movelist;
   mover->mpType->GenerateCaptures(movelist, mover);   
   mover->mpType->GenerateNonCaptures(movelist, mover);
   mpMoveGenerator->CheckForPromotions(movelist, mover->mColor);
   PostProcess(movelist, mover->mColor);

   for (i=0; i < movelist.size(); i++) {
      if (movelist[i] == moverec) {
         movePtr = new Move( movelist[i] );
         break;
      }
   }

   return movePtr;   
}

//------------------------------------------------------------------------
//       Class:  ChuShogi
//      Method:  GetSecondSquares
// Description:  Return a vector of integers representing possible
//               destination squares for the piece at the given 
//               board location when making a second part of a two
//               part move
//------------------------------------------------------------------------
void ChuShogi::GetSecondSquares(int start, int middle, vector<int>& squares)
{
   squares.clear();
   
   Piece* mover = mpBoard->mSquares[start];
   if (!mover) return;

   int type = mover->mpType->mTypeValue;
   if ((type != LION) && (type != SOARING_EAGLE) && 
       (type != HORNED_FALCON)) {
      return;
   }

   vector<int> final_squares;
   int factor = (mover->mColor == BLACK) ? 1 : -1;

   switch (type) {
   case LION:
      final_squares.push_back(middle-WIDTH-1);
      final_squares.push_back(middle-WIDTH);
      final_squares.push_back(middle-WIDTH+1);
      final_squares.push_back(middle-1);
      final_squares.push_back(middle+1);
      final_squares.push_back(middle+WIDTH-1);
      final_squares.push_back(middle+WIDTH);
      final_squares.push_back(middle+WIDTH+1);
      break;
   case SOARING_EAGLE:
      final_squares.push_back(middle - factor*(WIDTH+1));
      final_squares.push_back(middle - factor*(WIDTH-1));
      break;
   case HORNED_FALCON:
      final_squares.push_back(middle - factor*WIDTH);
      break;
   }

   vector<Move> movelist;
   mover->mpType->GenerateCaptures(movelist, mover);
   mover->mpType->GenerateNonCaptures(movelist, mover);
   
   // Allow any game specific processing
   int color = mover->mColor;
   PostProcess(movelist, color);

   Move* movePtr;
   unsigned j;
   squares.push_back(start);
   for (unsigned int i = 0; i < movelist.size(); i++) {
      movePtr = &(movelist[i]);
      if (!movePtr->mValidate || IsValid(movePtr, color)) {
         for (j = 0; j < final_squares.size(); j++) {
            if (final_squares[j] == movelist[i].mDestination) {
               squares.push_back(movelist[i].mDestination);
               break;
            }
         }
      }
   }   
}
