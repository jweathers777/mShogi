////////////////////////////////////////////////////////////////////////////
// Name:         TenjikuShogi.cpp
// Description:  Interface for a class that represents the 
//               game of Tenjiku Shogi
// Created:      08/27/2004 06:15:34 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:17:46 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// STL declaration
#include <vector>

// mShogi header files
#include "common.hpp"
#include "TenjikuShogi.hpp"
#include "TenjikuBoard.hpp"
#include "MoveRec.hpp"
#include "MoveGenerator.hpp"
#include "Evaluator.hpp"
#include "Notator.hpp"

// Tenjiku Shogi Pieces header files
#include "Pawn.hpp"
#include "Dog.hpp"
#include "Knight.hpp"
#include "IronGeneral.hpp"
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
#include "SideSoldier.hpp"
#include "VerticalMover.hpp"
#include "VerticalSoldier.hpp"
#include "Bishop.hpp"
#include "Rook.hpp"
#include "DragonHorse.hpp"
#include "DragonKing.hpp"
#include "ChariotSoldier.hpp"
#include "HornedFalcon.hpp"
#include "SoaringEagle.hpp"
#include "FreeKing.hpp"
#include "FreeEagle.hpp"
#include "WaterBuffalo.hpp"
#include "Lion.hpp"
#include "LionHawk.hpp"
#include "BishopGeneral.hpp"
#include "RookGeneral.hpp"
#include "ViceGeneral.hpp"
#include "GreatGeneral.hpp"
#include "FireDemon.hpp"
#include "King.hpp"
#include "Jewel.hpp"
#include "Tokin.hpp"
#include "MultiGeneral.hpp"
#include "FlyingStag.hpp"
#include "CrownPrince.hpp"
#include "Whale.hpp"
#include "WhiteHorse.hpp"
#include "FreeBoar.hpp"
#include "FlyingOx.hpp"
#include "HeavenlyTetrarchs.hpp"

// Define useful shorthand for describing initial board lay out
#define bK   TenjikuShogi::JEWEL + 1
#define bFiD TenjikuShogi::FIRE_DEMON + 1
#define bGGn TenjikuShogi::GREAT_GENERAL + 1
#define bVGn TenjikuShogi::VICE_GENERAL + 1
#define bRGn TenjikuShogi::ROOK_GENERAL + 1
#define bBGn TenjikuShogi::BISHOP_GENERAL + 1
#define bLHk TenjikuShogi::LION_HAWK + 1
#define bLn  TenjikuShogi::LION + 1
#define bWBf TenjikuShogi::WATER_BUFFALO + 1
#define bFEg TenjikuShogi::FREE_EAGLE + 1
#define bFK  TenjikuShogi::FREE_KING + 1
#define bSE  TenjikuShogi::SOARING_EAGLE + 1
#define bHF  TenjikuShogi::HORNED_FALCON + 1
#define bChS TenjikuShogi::CHARIOT_SOLDIER + 1
#define bDK  TenjikuShogi::DRAGON_KING + 1
#define bDH  TenjikuShogi::DRAGON_HORSE + 1
#define bR   TenjikuShogi::ROOK + 1
#define bB   TenjikuShogi::BISHOP + 1
#define bVSo TenjikuShogi::VERTICAL_SOLDIER + 1
#define bVM  TenjikuShogi::VERTICAL_MOVER + 1
#define bSSo TenjikuShogi::SIDE_SOLDIER + 1
#define bSM  TenjikuShogi::SIDE_MOVER + 1
#define bPh  TenjikuShogi::PHOENIX + 1
#define bKy  TenjikuShogi::KYLIN + 1
#define bRC  TenjikuShogi::REVERSE_CHARIOT + 1
#define bL   TenjikuShogi::LANCE + 1
#define bG   TenjikuShogi::GOLD_GENERAL + 1
#define bDE  TenjikuShogi::DRUNK_ELEPHANT + 1
#define bBT  TenjikuShogi::BLIND_TIGER + 1
#define bFL  TenjikuShogi::FEROCIOUS_LEOPARD + 1
#define bS   TenjikuShogi::SILVER_GENERAL + 1
#define bC   TenjikuShogi::COPPER_GENERAL + 1
#define bI   TenjikuShogi::IRON_GENERAL + 1
#define bN   TenjikuShogi::KNIGHT + 1
#define bDg  TenjikuShogi::DOG + 1
#define bP   TenjikuShogi::PAWN + 1

#define wK   TenjikuShogi::KING  + 38
#define wFiD TenjikuShogi::FIRE_DEMON  + 38
#define wGGn TenjikuShogi::GREAT_GENERAL  + 38
#define wVGn TenjikuShogi::VICE_GENERAL  + 38
#define wRGn TenjikuShogi::ROOK_GENERAL  + 38
#define wBGn TenjikuShogi::BISHOP_GENERAL  + 38
#define wLHk TenjikuShogi::LION_HAWK  + 38
#define wLn  TenjikuShogi::LION  + 38
#define wWBf TenjikuShogi::WATER_BUFFALO  + 38
#define wFEg TenjikuShogi::FREE_EAGLE  + 38
#define wFK  TenjikuShogi::FREE_KING  + 38
#define wSE  TenjikuShogi::SOARING_EAGLE  + 38
#define wHF  TenjikuShogi::HORNED_FALCON  + 38
#define wChS TenjikuShogi::CHARIOT_SOLDIER  + 38
#define wDK  TenjikuShogi::DRAGON_KING  + 38
#define wDH  TenjikuShogi::DRAGON_HORSE  + 38
#define wR   TenjikuShogi::ROOK  + 38
#define wB   TenjikuShogi::BISHOP  + 38
#define wVSo TenjikuShogi::VERTICAL_SOLDIER  + 38
#define wVM  TenjikuShogi::VERTICAL_MOVER  + 38
#define wSSo TenjikuShogi::SIDE_SOLDIER  + 38
#define wSM  TenjikuShogi::SIDE_MOVER  + 38
#define wPh  TenjikuShogi::PHOENIX  + 38
#define wKy  TenjikuShogi::KYLIN  + 38
#define wRC  TenjikuShogi::REVERSE_CHARIOT  + 38
#define wL   TenjikuShogi::LANCE  + 38
#define wG   TenjikuShogi::GOLD_GENERAL  + 38
#define wDE  TenjikuShogi::DRUNK_ELEPHANT  + 38
#define wBT  TenjikuShogi::BLIND_TIGER  + 38
#define wFL  TenjikuShogi::FEROCIOUS_LEOPARD  + 38
#define wS   TenjikuShogi::SILVER_GENERAL  + 38
#define wC   TenjikuShogi::COPPER_GENERAL  + 38
#define wI   TenjikuShogi::IRON_GENERAL  + 38
#define wN   TenjikuShogi::KNIGHT  + 38
#define wDg  TenjikuShogi::DOG  + 38
#define wP   TenjikuShogi::PAWN  + 38

using std::vector;
using std::map;
using std::make_pair;

// Constants
const int TenjikuShogi::WIDTH;
const int TenjikuShogi::HEIGHT;
const int TenjikuShogi::SIZE;
const int TenjikuShogi::PIECE_COUNT;
const int TenjikuShogi::PIECE_TYPES;

// Array that defines the inital board lay out
const int TenjikuShogi::mStartingPattern[] = {
   wL,   wN,   wFL,  wI,   wC,   wS,   wG,   wK,   
   wDE,  wG,   wS,   wC,   wI,   wFL,  wN,   wL,
   wRC,  0,    wChS, wChS, 0,    wBT,  wKy,  wLn,
   wFK,  wPh,  wBT,  0,    wChS, wChS, 0,    wRC,
   wSSo, wVSo, wB,   wDH,  wDK,  wWBf, wFiD, wLHk,
   wFEg, wFiD, wWBf, wDK,  wDH,  wB,   wVSo, wSSo,
   wSM,  wVM,  wR,   wHF,  wSE,  wBGn, wRGn, wGGn,
   wVGn, wRGn, wBGn, wSE,  wHF,  wR,   wVM,  wSM,
   wP,   wP  , wP,   wP,   wP,   wP,   wP,   wP,   
   wP,   wP  , wP,   wP,   wP,   wP,   wP,   wP,   
   0,    0,    0,    0,    wDg,  0,    0,    0,
   0,    0,    0,    wDg,  0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    0,    0,    0,    0,
   0,    0,    0,    0,    bDg,  0,    0,    0,
   0,    0,    0,    bDg,  0,    0,    0,    0,
   bP,   bP  , bP,   bP,   bP,   bP,   bP,   bP,   
   bP,   bP  , bP,   bP,   bP,   bP,   bP,   bP,   
   bSM,  bVM,  bR,   bHF,  bSE,  bBGn, bRGn, bVGn,
   bGGn, bRGn, bBGn, bSE,  bHF,  bR,   bVM,  bSM,
   bSSo, bVSo, bB,   bDH,  bDK,  bWBf, bFiD, bFEg,
   bLHk, bFiD, bWBf, bDK,  bDH,  bB,   bVSo, bSSo,
   bRC,  0,    bChS, bChS, 0,    bBT,  bPh,  bFK,
   bLn,  bKy,  bBT,  0,    bChS, bChS, 0,    bRC,
   bL,   bN,   bFL,  bI,   bC,   bS,   bG,   bDE,   
   bK,   bG,   bS,   bC,   bI,   bFL,  bN,   bL
};

// Array that defines promotions
const int TenjikuShogi::mPromotionMap[] = {
  // PAWN,  DOG,           KNIGHT,       IRON_GENERAL,     COPPER_GENERAL,
     TOKIN, MULTI_GENERAL, SIDE_SOLDIER, VERTICAL_SOLDIER, SIDE_MOVER,
  // SILVER_GENERAL, FEROCIOUS_LEOPARD, BLIND_TIGER, 
     VERTICAL_MOVER, BISHOP,            FLYING_STAG, 
  // DRUNK_ELEPHANT, GOLD_GENERAL, LANCE,       REVERSE_CHARIOT,
     CROWN_PRINCE,   ROOK,         WHITE_HORSE, WHALE,
  // KYLIN, PHOENIX,   SIDE_MOVER, SIDE_SOLDIER,
     LION,  FREE_KING, FREE_BOAR,  WATER_BUFFALO,
  // VERTICAL_MOVER, VERTICAL_SOLDIER,
     FLYING_OX,      CHARIOT_SOLDIER,
  // BISHOP,       ROOK,        DRAGON_HORSE,  DRAGON_KING,
     DRAGON_HORSE, DRAGON_KING, HORNED_FALCON, SOARING_EAGLE,
  // CHARIOT_SOLDIER,    HORNED_FALCON,  SOARING_EAGLE, 
     HEAVENLY_TETRARCHS, BISHOP_GENERAL, ROOK_GENERAL, 
  // FREE_KING,  FREE_EAGLE,   WATER_BUFFALO,
     FREE_EAGLE, NO_PROMOTION, FIRE_DEMON,
  // LION,      LION_HAWK,    BISHOP_GENERAL, ROOK_GENERAL, 
     LION_HAWK, NO_PROMOTION, VICE_GENERAL,   GREAT_GENERAL, 
  // VICE_GENERAL, GREAT_GENERAL, FIRE_DEMON,   KING,         JEWEL
     NO_PROMOTION, NO_PROMOTION,  NO_PROMOTION, NO_PROMOTION, NO_PROMOTION
};

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  TenjikuShogi
// Description:  Create an instance of a tenjiku shogi game
//--------------------------------------------------------------------------
TenjikuShogi::TenjikuShogi()
{
   // Create a 16x16 board
   mpBoard = new TenjikuBoard(WIDTH, HEIGHT, 5, PIECE_TYPES);
   mpTenjikuBoard = (TenjikuBoard*)mpBoard;

   // Create the piece types that we need
   mPieceTypes.push_back( new Pawn(mpBoard, 1000, PAWN) );
   mPieceTypes.push_back( new Dog(mpBoard, 1200, DOG) );
   mPieceTypes.push_back( new Knight(mpBoard, 1371, KNIGHT) );
   mPieceTypes.push_back( new IronGeneral(mpBoard, 1892, IRON_GENERAL) );
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
   mPieceTypes.push_back( new SideSoldier(mpBoard, 5453, SIDE_SOLDIER) );
   mPieceTypes.push_back( new VerticalSoldier(mpBoard, 5535, VERTICAL_SOLDIER) );
   mPieceTypes.push_back( new VerticalMover(mpBoard, 5620, VERTICAL_MOVER) );
   mPieceTypes.push_back( new Bishop(mpBoard, 8644, BISHOP) );
   mPieceTypes.push_back( new Rook(mpBoard, 8776, ROOK) );
   mPieceTypes.push_back( new DragonHorse(mpBoard, 11456, DRAGON_HORSE) );
   mPieceTypes.push_back( new DragonKing(mpBoard, 11631, DRAGON_KING) );
   mPieceTypes.push_back( new ChariotSoldier(mpBoard, 12233, CHARIOT_SOLDIER) );
   mPieceTypes.push_back( new HornedFalcon(mpBoard, 12451, HORNED_FALCON) );
   mPieceTypes.push_back( new SoaringEagle(mpBoard, 12672, SOARING_EAGLE) );
   mPieceTypes.push_back( new FreeKing(mpBoard, 13800, FREE_KING) );
   mPieceTypes.push_back( new FreeEagle(mpBoard, 14027, FREE_EAGLE) );
   mPieceTypes.push_back( new WaterBuffalo(mpBoard, 14300, WATER_BUFFALO) );
   mPieceTypes.push_back( new Lion(mpBoard, 14578, LION) );
   mPieceTypes.push_back( new LionHawk(mpBoard, 14817, LION_HAWK) );
   mPieceTypes.push_back( new BishopGeneral(mpBoard, 15182, BISHOP_GENERAL,
                                            0, &mRankMap) );
   mPieceTypes.push_back( new RookGeneral(mpBoard, 15415, ROOK_GENERAL,
                                          1, &mRankMap) );
   mPieceTypes.push_back( new ViceGeneral(mpBoard, 18120, VICE_GENERAL,
                                          2, &mRankMap) );
   mPieceTypes.push_back( new GreatGeneral(mpBoard, 18398, GREAT_GENERAL,
                                           3, &mRankMap) );
   PieceType* typePtr = new FireDemon(mpBoard, 55194, FIRE_DEMON);
   mPieceTypes.push_back( typePtr );
   mPieceTypes.push_back( new King(mpBoard, 650000, KING) );
   mPieceTypes.push_back( new Jewel(mpBoard, 650000, JEWEL) );
  
   mRankMap.insert(make_pair((int)KING, 4));
   mRankMap.insert(make_pair((int)JEWEL, 4));
   
   mPieceTypes.push_back( new Tokin(mpBoard, 1501, TOKIN) );
   mPieceTypes.push_back( new MultiGeneral(mpBoard, 5494, MULTI_GENERAL) );
   mPieceTypes.push_back( new FlyingStag(mpBoard, 4810, FLYING_STAG) );
   mPieceTypes.push_back( new CrownPrince(mpBoard, 5135, CROWN_PRINCE) );
   mPieceTypes.push_back( new Whale(mpBoard, 5240, WHALE) );
   mPieceTypes.push_back( new WhiteHorse(mpBoard, 5276, WHITE_HORSE) );
   mPieceTypes.push_back( new FreeBoar(mpBoard, 9234, FREE_BOAR) );
   mPieceTypes.push_back( new FlyingOx(mpBoard, 9375, FLYING_OX) );
   mPieceTypes.push_back( new HeavenlyTetrarchs(mpBoard, 13236, HEAVENLY_TETRARCHS) );
   
   // Assign positional weights to the piece types
   SetWeights();
   
   // Create pieces and place them on the board
   Reset();

   // Dis-allow drops in chu shogi
   mAllowDrops = false;

   // Initialize pointer to the fire demon burn bitboards
   mpTenjikuBoard->mpBurnBitboards = ((FireDemon*)(typePtr))->mBurnBitboards;

   // Initialize board's record of Fire Demon piece type value
   mpTenjikuBoard->mFireDemonValue = FIRE_DEMON;

   // Initialize the burn zones
   mpTenjikuBoard->mBurnZones[0].resize(SIZE);
   mpTenjikuBoard->mBurnZones[1].resize(SIZE);
   mpTenjikuBoard->UpdateBurnZones();
   
   // Create a new move generator
   mpMoveGenerator = new MoveGenerator(this, PIECE_COUNT);
   
   // Create a new evaluator
   mpEvaluator = new Evaluator(mPieces);
   
   // Create a new Notator
   mpNotator = new Notator(mpBoard);

   // Grab a pointer to the instance of the area mover object
   mpAreaMover = AreaMover::GetInstance(mpBoard);
}

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  ~TenjikuShogi
// Description:  Destroy this instance of a mini shogi game
//--------------------------------------------------------------------------
TenjikuShogi::~TenjikuShogi()
{
   for (unsigned int i = 0; i < mPieceTypes.size(); i++) {
      if (mPieceTypes[i]) {
         delete mPieceTypes[i];
      }
   }
}

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  IsLegal
// Description:  Returns true if the given move for the given side is legal
//--------------------------------------------------------------------------
bool TenjikuShogi::IsLegal(Move* move, int side)
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

   // Is this a suicidal move?
   if ( mpTenjikuBoard->mBurnZones[side].test(move->mDestination) ) {
      move->mSuicide = true;
   }

   // Can this piece actually make this move?
   vector<Move> movelist;
   mover->mpType->GenerateCaptures(movelist, mover);   
   mover->mpType->GenerateNonCaptures(movelist, mover);
   mpMoveGenerator->CheckForPromotions(movelist, side);
   PostProcess(movelist, side);

   for (i=0; i < movelist.size(); i++) {
      if (movelist[i] == *move) {
         if (!movelist[i].mValidate || IsValid(&(movelist[i]), side)) {
            move->mCaptures = movelist[i].mCaptures;
            return true;
         }
         else
            return false;
      }
   }

   // No matching legal move was found
   return false;   
}

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  IsValid
// Description:  Returns true if the given pseudo-legal move for the 
//               given side is truly legal
//--------------------------------------------------------------------------
bool TenjikuShogi::IsValid(Move* move, int side)
{
   return mpAreaMover->HasLegalPath(move->mSource, move->mDestination);
}

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  CanPromote
// Description:  Returns true if the given move offers the 
//               opportunity for promotion 
//--------------------------------------------------------------------------
bool TenjikuShogi::CanPromote(Move* move)
{
   vector<Move> movelist;
   int side = move->mpMover->mColor;

   movelist.push_back(*move);
   mpMoveGenerator->CheckForPromotions(movelist, side);
   
   return (movelist.size() > 1);
}

//------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  CanMoveTwice
// Description:  Returns true if this move offers the opportunity to move
//               twice in one move
//------------------------------------------------------------------------
bool TenjikuShogi::CanMoveTwice(int start, int finish)
{
   int delta;
   int factor;
   bool result = false;
   
   if (mpBoard->mSquares[start]) {
      int factor = (mpBoard->mSquares[start]->mColor == BLACK) ? -1 : 1;
      switch (mpBoard->mSquares[start]->mpType->mTypeValue) {
      case LION:
      case LION_HAWK:
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
//       Class:  TenjikuShogi
//      Method:  GameWon
// Description:  Returns true if the current game state is a winning state
//--------------------------------------------------------------------------
bool TenjikuShogi::GameWon(int side)
{
   int enemy = 1-side;
   if (mpCrownPrinces[enemy])
      return (mpCrownPrinces[enemy]->mCaptured && 
              mpKings[enemy]->mCaptured);
   else
      return mpKings[enemy]->mCaptured;
}

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  GameDrawn
// Description:  Returns true if the current game state is drawn
//--------------------------------------------------------------------------
bool TenjikuShogi::GameDrawn()
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
//       Class:  TenjikuShogi
//      Method:  PostProcess
// Description:  Offers opportunity for game specific post processing
//               of a generated list of moves
//--------------------------------------------------------------------------
void TenjikuShogi::PostProcess(vector<Move>& movelist, int side) 
{
   vector<Move>::size_type index, size;

   size = movelist.size();
   vector<Move> newlist;
   for (index = 0; index < size; index++) {
      if (mpTenjikuBoard->mBurnZones[side].test(
                  movelist[index].mDestination) ) {
         movelist[index].mSuicide = true;
      }

      // Add this move to new list that has only legal moves
      newlist.push_back(movelist[index]);
   }
   movelist.clear();
   movelist.insert(movelist.begin(), newlist.begin(), newlist.end());
}

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  ForcePromotion
// Description:  Returns true if this move forces a promotion
//--------------------------------------------------------------------------
bool TenjikuShogi::ForcePromotion(const Move& move) 
{
   //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
   // Needs to be implemented
   //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
   return false;
}

//--------------------------------------------------------------------------
//       Class:  TenjikuShogi
//      Method:  SetWeights
// Description:  Assign positional weight matrices to the piece types
//--------------------------------------------------------------------------
void TenjikuShogi::SetWeights()
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
//       Class:  TenjikuShogi
//      Method:  Reset
// Description:  Resets the game to its initial state
//--------------------------------------------------------------------------
void TenjikuShogi::Reset() 
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

   // Clear the list of fire demons for each side
   mpTenjikuBoard->mpFireDemons[0].clear();
   mpTenjikuBoard->mpFireDemons[1].clear();

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
 
         // Establish pointers to the fire demons for 
         // quick checks for suicide moves
         if (piecetype == FIRE_DEMON) {
            mpTenjikuBoard->mpFireDemons[color].push_back(
               mpBoard->mSquares[square]);
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
//       Class:  TenjikuShogi
//      Method:  CreateMove
// Description:  Returns a pointer to a new move constructed 
//               from a move record
//--------------------------------------------------------------------------
Move* TenjikuShogi::CreateMove(const MoveRec& moverec)
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
//       Class:  TenjikuShogi
//      Method:  GetSecondSquares
// Description:  Return a vector of integers representing possible
//               destination squares for the piece at the given 
//               board location when making a second part of a two
//               part move
//------------------------------------------------------------------------
void TenjikuShogi::GetSecondSquares(int start, int middle, vector<int>& squares)
{
   squares.clear();
   
   Piece* mover = mpBoard->mSquares[start];
   if (!mover) return;

   int type = mover->mpType->mTypeValue;
   if ((type != LION) && (type != LION_HAWK) &&
       (type != SOARING_EAGLE) && (type != HORNED_FALCON)) {
      return;
   }

   vector<int> final_squares;
   int factor = (mover->mColor == BLACK) ? 1 : -1;

   switch (type) {
   case LION:
   case LION_HAWK:
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

