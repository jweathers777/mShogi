////////////////////////////////////////////////////////////////////////////
// Name:         TenjikuShogi.hpp
// Description:  Interface for a class that represents the 
//               game of Tenjiku Shogi
// Created:      08/27/2004 05:56:09 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 22:17:46 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __TenjikuShogi_HPP__
#define __TenjikuShogi_HPP__

// STL declaration
#include <map>

// Local header files
#include "Game.hpp"

class AreaMover;
class TenjikuBoard;

class TenjikuShogi : public Game
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   TenjikuShogi();
   virtual ~TenjikuShogi();

   // ======================================================================
   // Operations
   // ======================================================================
   bool IsLegal(Move* move, int side);
   bool IsValid(Move* move, int side);
   bool CanPromote(Move* move);
   bool CanMoveTwice(int start, int finish);
   bool GameWon(int side);
   bool GameDrawn();
   
   void PostProcess(std::vector<Move>& movelist, int side);
   
   int  GetPromotedType(int piecetype) 
   {
      if (piecetype >= UNPROMOTED_PIECE_TYPES)
         return -1;
      else if (mPromotionMap[piecetype] == NO_PROMOTION)
         return -1;
      else
         return mPromotionMap[piecetype];
   }

   bool ForcePromotion(const Move& move);

   void SetWeights();
   void Reset();
   
   Move* CreateMove(const MoveRec& moverec);
   void GetSecondSquares(int start, int middle, std::vector<int>& squares);

private:
   // ======================================================================
   // Constants
   // ======================================================================
   static const int WIDTH = 16;
   static const int HEIGHT = 16;
   static const int SIZE = 256;
   static const int PIECE_COUNT = 78;
   static const int PIECE_TYPES = 46;
   static const int UNPROMOTED_PIECE_TYPES = 37;
   
   enum {
      PAWN=0, DOG,
      KNIGHT, IRON_GENERAL, COPPER_GENERAL,
      SILVER_GENERAL, FEROCIOUS_LEOPARD, BLIND_TIGER, 
      DRUNK_ELEPHANT, GOLD_GENERAL,
      LANCE, REVERSE_CHARIOT,
      KYLIN, PHOENIX, SIDE_MOVER, SIDE_SOLDIER,
      VERTICAL_MOVER, VERTICAL_SOLDIER,
      BISHOP, ROOK,
      DRAGON_HORSE, DRAGON_KING,
      CHARIOT_SOLDIER, HORNED_FALCON, SOARING_EAGLE, FREE_KING, FREE_EAGLE,
      WATER_BUFFALO,
      LION, LION_HAWK,
      BISHOP_GENERAL, ROOK_GENERAL,
      VICE_GENERAL, GREAT_GENERAL,
      FIRE_DEMON,
      KING, JEWEL,
      TOKIN, MULTI_GENERAL, FLYING_STAG, 
      CROWN_PRINCE, WHALE, WHITE_HORSE,
      FREE_BOAR, FLYING_OX, HEAVENLY_TETRARCHS, NO_PROMOTION
   };
   
   static const int mStartingPattern[SIZE];
   static const int mPromotionMap[UNPROMOTED_PIECE_TYPES];
   
   // ======================================================================
   // Data
   // ======================================================================
   std::map<int,int> mRankMap; // Mapping of piece type values to piece ranks
   TenjikuBoard* mpTenjikuBoard; // Pointer to the board as a tenjiku board

   AreaMover* mpAreaMover;   // The area mover object
   Piece* mpKings[2];        // Pointers to the kings
   Piece* mpCrownPrinces[2]; // Pointer to the crown princes
};

#endif // __TenjikuShogi_HPP__
