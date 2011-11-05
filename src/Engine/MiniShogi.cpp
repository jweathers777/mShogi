////////////////////////////////////////////////////////////////////////////////
// Name:         MiniShogi.cpp
// Description:  Implementation for a class that represents the game Mini Shogi
// Created:      Apr 20, 2004 5:06:11 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// STL declaration
#include <vector>

// mShogi header files
#include "common.hpp"
#include "MiniShogi.hpp"
#include "Board.hpp"
#include "MoveRec.hpp"
#include "MoveGenerator.hpp"
#include "Evaluator.hpp"
#include "Notator.hpp"

// Mini Shogi Pieces header files
#include "Pawn.hpp"
#include "SilverGeneral.hpp"
#include "GoldGeneral.hpp"
#include "Bishop.hpp"
#include "Rook.hpp"
#include "Jewel.hpp"
#include "King.hpp"
#include "Tokin.hpp"
#include "PromotedSilver.hpp"
#include "DragonHorse.hpp"
#include "DragonKing.hpp"

// Define useful shorthand for describing initial board lay out
#define bK MiniShogi::JEWEL + 1
#define bR MiniShogi::ROOK + 1
#define bB MiniShogi::BISHOP + 1
#define bG MiniShogi::GOLD_GENERAL + 1
#define bS MiniShogi::SILVER_GENERAL + 1
#define bP MiniShogi::PAWN + 1

#define wK MiniShogi::KING + 8
#define wR MiniShogi::ROOK + 8
#define wB MiniShogi::BISHOP + 8
#define wG MiniShogi::GOLD_GENERAL + 8
#define wS MiniShogi::SILVER_GENERAL + 8
#define wP MiniShogi::PAWN + 8

using std::vector;

// Constants
const int MiniShogi::WIDTH;
const int MiniShogi::HEIGHT;
const int MiniShogi::SIZE;
const int MiniShogi::PIECE_COUNT;
const int MiniShogi::PIECE_TYPES;

// Array that defines the inital board lay out
const int MiniShogi::mStartingPattern[] = {
   wK, wG, wS, wB, wR,
   wP, 0,  0,  0,  0,
   0,  0,  0,  0,  0,
   0,  0,  0,  0,  bP,
   bR, bB, bS, bG, bK
};

// Array that defines promotions
const int MiniShogi::mPromotionMap[] = {
  // PAWN, SILVER_GENERAL, GOLD_GENERAL,  
     TOKIN, PROMOTED_SILVER, NO_PROMOTION, 
  // BISHOP, ROOK, KING, JEWEL
     DRAGON_HORSE, DRAGON_KING, NO_PROMOTION, NO_PROMOTION
};

#if 1
// Arrays that define positional weights
const int MiniShogi::mPieceWeights[PIECE_TYPES][SIZE] = { 
   {  // Pawn weights
      23,  13,  12,  19,  59,
      13,   5,   3,   9,  20,
       0,  -9, -12,  -9,   0,
      -6, -17, -21, -19, -10,
       2, -12, -16, -15, -18
   },
   {  // Silver General Weights
       -30,   81,  106,   87,  -20,
        93,  293,  297,  312,    0,
       -38,   63,   79,   15,  -34,
      -143,  -93,  -92,  -93, -143,
      -172, -166, -115, -114, -159
   },
   {  // Gold General Weights
      -328, -185, -178, -178, -186,
       -62,  226,  220,  211,   44,
       -55,  207,  223,  223,   15,
       -97,  176,  188,  131, -105,
       -227,   9,   18,  -46, -233
   },
   {  // Bishop Weights
       84,  85, 128,  107,  80,
      -46,  89, 100,   95, -50,
      -65,  46, 156,    4, -69,
      -87,  -3, -74,  -18, -94,
      -46, -98, -95, -145, -63
   },
   {  // Rook Weights
      -75, 106,  94,  84,   24,
       44,  46,  45,  42,   17,
       -9,  34,  36,  30,    6,
      -53,  16,  11,   5,   26,
     -145, -49, -56, -59, -213
   },
   {  // King Weights
     -141,  27,  42,   38, -156,
       33, 133, 129,  141,   44,
       19, 124, 124,  111,   -9,
      -53,  56,  52,  -15,  -52,
     -205, -56, -66, -120, -187
   },
   {  // Jewel Weights
     -141,  27,  42,   38, -156,
       33, 133, 129,  141,   44,
       19, 124, 124,  111,   -9,
      -53,  56,  52,  -15,  -52,
     -205, -56, -66, -120, -187
   },
   {  // Tokin Weights
      -308, -173, -173, -156, -301,
       -51,  235,  233,  221,  -35,
       -52,  226,  223,  227,  -90,
       -71,  206,  185,  147,  -73,
      -225,   25,   19,  -23, -213
   },
   {  // Promoted Silver Weights
      -305, -173, -152, -175, -320,
       -43,  234,  231,  224,  -30,
       -32,  239,  220,  243,  -85,
       -85,  202,  193,  149,  -75,
      -212,    9,    7,  -27, -218
   },
   {  // Dragon Horse Weights
      -105, -24,   7,    3, -108,
       -23, 104, 118,  123,  -13,
       -13, 111, 154,  112,  -72,
       -72,  87,  86,   73,  -69,
       -99, -54, -56, -105, -152
   },
   {  // Dragon King Weights
      -167,   6,  15,  -1, -129,
        46,  74,  79,  74,   29,
        37,  72,  85,  57,   31,
       -22,  59,  49,  55,   17,
      -131, -17, -17, -69, -225
   }
};
#else
// Arrays that define positional weights
const int MiniShogi::mPieceWeights[PIECE_TYPES][SIZE] = { 
   {  // Pawn weights
      0,  0,  0,  0,  0,
      0,   0,   0,   0,  0,
       0,  0, 0,  0,   0,
      0, 0, 0, 0, 0,
       0, 0, 0, 0, 0
   },
   {  // Silver General Weights
       0,   0,  0,   0,  0,
        0,  0,  0,  0,    0,
       0,   0,   0,   0,  0,
      0,  0,  0,  0, 0,
      0, 0, 0, 0, 0
   },
   {  // Gold General Weights
      0, 0, 0, 0, 0,
       0,  0,  0,  0,   0,
       0,  0,  0,  0,   0,
       0,  0,  0,  0, 0,
       0,   0,   0,  0, 0
   },
   {  // Bishop Weights
       0,  0, 0,  0,  0,
      0,  0, 0,   0, 0,
      0,  0, 0,    0, 0,
      0,  0, 0,  0, 0,
      0, 0, 0, 0, 0
   },
   {  // Rook Weights
      0, 0,  0,  0,   0,
       0,  0,  0,  0,   0,
       0,  0,  0,  0,    0,
      0,  0,  0,   0,   0,
     0, 0, 0, 0, 0
   },
   {  // King Weights
     0,  0,  0,   0, 0,
       0, 0, 0,  0,   0,
       0, 0, 0,  0,   0,
      0,  0,  0,  0,  0,
     0, 0, 0, 0, 0
   },
   {  // Jewel Weights
     0,  0,  0,   0, 0,
       0, 0, 0,  0,   0,
       0, 0, 0,  0,   0,
      0,  0,  0,  0,  0,
     0, 0, 0, 0, 0
   },
   {  // Tokin Weights
      0, 0, 0, 0, 0,
       0,  0,  0,  0,  0,
       0,  0,  0,  0,  0,
       0,  0,  0,  0,  0,
      0,   0,   0,  0, 0
   },
   {  // Promoted Silver Weights
      0, 0, 0, 0, 0,
       0,  0,  0,  0,  0,
       0,  0,  0,  0,  0,
       0,  0,  0,  0,  0,
      0,    0,    0,  0, 0
   },
   {  // Dragon Horse Weights
      0, 0,   0,    0, 0,
       0, 0, 0,  0,  0,
       0, 0, 0,  0,  0,
       0,  0,  0,   0,  0,
       0, 0, 0, 0, 0
   },
   {  // Dragon King Weights
      0,   0,  0,  0, 0,
        0,  0,  0,  0,   0,
        0,  0,  0,  0,   0,
       0,  0,  0,  0,   0,
      0, 0, 0, 0, 0
   }
};

#endif

//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  MiniShogi
// Description:  Create an instance of a mini shogi game
//--------------------------------------------------------------------------
MiniShogi::MiniShogi()
{
   // Create a 5x5 board
   mpBoard = new Board(WIDTH, HEIGHT, 1, PIECE_TYPES);

#if 1   
   // Create the piece types that we need
   mPieceTypes.push_back( new Pawn(mpBoard, 1509, PAWN) );
   mPieceTypes.push_back( new SilverGeneral(mpBoard, 2797, SILVER_GENERAL) );
   mPieceTypes.push_back( new GoldGeneral(mpBoard, 2631, GOLD_GENERAL) );
   mPieceTypes.push_back( new Bishop(mpBoard, 3317, BISHOP) );
   mPieceTypes.push_back( new Rook(mpBoard, 3825, ROOK) );
   mPieceTypes.push_back( new King(mpBoard, 101142, KING) );
   mPieceTypes.push_back( new Jewel(mpBoard, 101142, JEWEL) );
   mPieceTypes.push_back( new Tokin(mpBoard, 2266, TOKIN) );
   mPieceTypes.push_back( new PromotedSilver(mpBoard, 2618, PROMOTED_SILVER) );
   mPieceTypes.push_back( new DragonHorse(mpBoard, 3458, DRAGON_HORSE) );
   mPieceTypes.push_back( new DragonKing(mpBoard, 3856, DRAGON_KING) );
#else
   // Create the piece types that we need
   mPieceTypes.push_back( new Pawn(mpBoard, 1237, PAWN) );
   mPieceTypes.push_back( new SilverGeneral(mpBoard, 1696, SILVER_GENERAL) );
   mPieceTypes.push_back( new GoldGeneral(mpBoard, 1963, GOLD_GENERAL) );
   mPieceTypes.push_back( new Bishop(mpBoard, 2665, BISHOP) );
   mPieceTypes.push_back( new Rook(mpBoard, 4005, ROOK) );
   mPieceTypes.push_back( new King(mpBoard, 32422, KING) );
   mPieceTypes.push_back( new Jewel(mpBoard, 32422, JEWEL) );
   mPieceTypes.push_back( new Tokin(mpBoard, 1295, TOKIN) );
   mPieceTypes.push_back( new PromotedSilver(mpBoard, 2686, PROMOTED_SILVER) );
   mPieceTypes.push_back( new DragonHorse(mpBoard, 6875, DRAGON_HORSE) );
   mPieceTypes.push_back( new DragonKing(mpBoard, 10000, DRAGON_KING) ); 
#endif
   
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
}

//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  ~MiniShogi
// Description:  Destroy this instance of a mini shogi game
//--------------------------------------------------------------------------
MiniShogi::~MiniShogi()
{
   for (unsigned int i = 0; i < mPieceTypes.size(); i++) {
      if (mPieceTypes[i]) {
         delete mPieceTypes[i];
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  IsLegal
// Description:  Returns true if the given move for the given side is legal
//--------------------------------------------------------------------------
bool MiniShogi::IsLegal(Move* move, int side)
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

   for (i=0; i < movelist.size(); i++) {
      if (movelist[i] == *move) {
         return true;
      }
   }

   // No matching legal move was found
   return false;   
}

//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  CanPromote
// Description:  Returns true if the given move offers the 
//               opportunity for promotion 
//--------------------------------------------------------------------------
bool MiniShogi::CanPromote(Move* move)
{
   vector<Move> movelist;
   int side = move->mpMover->mColor;

   movelist.push_back(*move);
   mpMoveGenerator->CheckForPromotions(movelist, side);
   
   return (movelist.size() > 1);
}

//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  GameWon
// Description:  Returns true if the current game state is a winning state
//--------------------------------------------------------------------------
bool MiniShogi::GameWon(int side)
{
   return mpKings[1-side]->mCaptured;
}

//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  GameDrawn
// Description:  Returns true if the current game state is drawn
//--------------------------------------------------------------------------
bool MiniShogi::GameDrawn()
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

   // Check for draw by naked kings
   if (!drawn) {
      int pieces = 0;
      unsigned int i;
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
//       Class:  MiniShogi
//      Method:  ForcePromotion
// Description:  Returns true if this move forces a promotion
//--------------------------------------------------------------------------
bool MiniShogi::ForcePromotion(const Move& move) 
{
   Piece* mover = move.mpMover;
   return ((mover->mTypeValue == PAWN) && 
           (mpBoard->mPromotionZones[mover->mColor][move.mDestination]));
}

//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  SetWeights
// Description:  Assign positional weight matrices to the piece types
//--------------------------------------------------------------------------
void MiniShogi::SetWeights()
{
   for (unsigned int i=0; i < mPieceTypes.size(); i++) {
      mPieceTypes[i]->mWeights[0].resize(SIZE);
      mPieceTypes[i]->mWeights[1].resize(SIZE);
      for (int square = 0; square < SIZE; square++) {
         mPieceTypes[i]->mWeights[0][square] = mPieceWeights[i][square];
         mPieceTypes[i]->mWeights[1][square] = mPieceWeights[i][25-square];
      }
   }
}


//--------------------------------------------------------------------------
//       Class:  MiniShogi
//      Method:  Reset
// Description:  Resets the game to its initial state
//--------------------------------------------------------------------------
void MiniShogi::Reset() 
{
   int i, piecetype, color, promoted;
   
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
//       Class:  MiniShogi
//      Method:  CreateMove
// Description:  Returns a pointer to a new move constructed 
//               from a move record
//--------------------------------------------------------------------------
Move* MiniShogi::CreateMove(const MoveRec& moverec)
{
   int i;
   Piece* mover = (Piece*)moverec.mPiecePtr;
   Move* movePtr = 0;
   
   // Generate the possible moves that this piece can make
   vector<Move> movelist;
   mover->mpType->GenerateCaptures(movelist, mover);   
   mover->mpType->GenerateNonCaptures(movelist, mover);
   mpMoveGenerator->CheckForPromotions(movelist, mover->mColor);

   for (i=0; i < movelist.size(); i++) {
      if (movelist[i] == moverec) {
         movePtr = new Move( movelist[i] );
         break;
      }
   }

   return movePtr;   
}

