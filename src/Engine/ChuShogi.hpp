////////////////////////////////////////////////////////////////////////////
// Name:         ChuShogi.hpp
// Description:  Interface for a class that represents the 
//               game of Chu Shogi
// Created:      02/04/2005 11:10:31 Eastern Standard Time
// Last Updated: $Date$
// Revision:     $Revision$
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2005 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __ChuShogi_HPP__
#define __ChuShogi_HPP__

// Local header files
#include "Game.hpp"

class ChuShogi : public Game
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   ChuShogi();
   virtual ~ChuShogi();

   // ======================================================================
   // Operations
   // ======================================================================
   bool IsLegal(Move* move, int side);
   bool IsValid(Move* move, int side);
   bool CanPromote(Move* move);
   bool CanMoveTwice(int start, int finish);
   bool GameWon(int side);
   bool GameDrawn();
   
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
   static const int WIDTH = 12;
   static const int HEIGHT = 12;
   static const int SIZE = 144;
   static const int PIECE_COUNT = 46;
   static const int PIECE_TYPES = 31;
   static const int UNPROMOTED_PIECE_TYPES = 22;
   
   enum {
      PAWN=0, GO_BETWEEN,
      COPPER_GENERAL, SILVER_GENERAL, 
      FEROCIOUS_LEOPARD, BLIND_TIGER, 
      DRUNK_ELEPHANT, GOLD_GENERAL,
      LANCE, REVERSE_CHARIOT,
      KYLIN, PHOENIX, 
      SIDE_MOVER, VERTICAL_MOVER,
      BISHOP, ROOK,
      DRAGON_HORSE, DRAGON_KING,
      FREE_KING, LION,
      KING, JEWEL,
      TOKIN, FLYING_STAG, 
      CROWN_PRINCE, WHALE, WHITE_HORSE,
      FREE_BOAR, FLYING_OX, 
      HORNED_FALCON, SOARING_EAGLE, NO_PROMOTION
   };
   
   static const int mStartingPattern[SIZE];
   static const int mPromotionMap[UNPROMOTED_PIECE_TYPES];
   
   // ======================================================================
   // Data
   // ======================================================================
   Piece* mpKings[2];        // Pointers to the kings
   Piece* mpCrownPrinces[2]; // Pointer to the crown princes
};

#endif // __ChuShogi_HPP__

