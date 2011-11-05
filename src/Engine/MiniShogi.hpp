////////////////////////////////////////////////////////////////////////////////
// Name:         MiniShogi
// Description:  Interface for a class that represents the game of Mini Shogi
// Created:      Apr 20, 2004 5:05:09 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef MINISHOGI_HPP
#define MINISHOGI_HPP

#include "Game.hpp"

class MiniShogi : public Game
{
public:
   // ==========================================================================
   // Construction and Destruction
   // ==========================================================================
   MiniShogi();
   virtual ~MiniShogi();
   
   // ==========================================================================
   // Operations
   // ==========================================================================
   bool IsLegal(Move* move, int side);
   bool CanPromote(Move* move);
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
   
private:
   // ==========================================================================
   // Constants
   // ==========================================================================
   static const int WIDTH = 5;
   static const int HEIGHT = 5;
   static const int SIZE = 25;
   static const int PIECE_COUNT = 6;
   static const int PIECE_TYPES = 11;
   static const int UNPROMOTED_PIECE_TYPES = 7;
   
   enum {
     PAWN=0, SILVER_GENERAL, GOLD_GENERAL, BISHOP, ROOK, KING, JEWEL,
     TOKIN, PROMOTED_SILVER, DRAGON_HORSE, DRAGON_KING, NO_PROMOTION
   };
   
   static const int mStartingPattern[SIZE];
   static const int mPromotionMap[UNPROMOTED_PIECE_TYPES];
   
   static const int mPieceWeights[PIECE_TYPES][SIZE];
   
   Piece* mpKings[2];
};

#endif // MINISHOGI_HPP
