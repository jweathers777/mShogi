////////////////////////////////////////////////////////////////////////////////
// Name:         Game.hpp
// Description:  Interface for abstract class that represents a game
// Created:      Apr 20, 2004 10:18:12 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef GAME_HPP
#define GAME_HPP

// STL header files
#include <string>
#include <vector>

// mShogi header files
#include "common.hpp"
#include "PieceInfo.hpp"
#include "Piece.hpp"

// Forward declarations
struct MoveRec;
class Move;
class PieceType;
class Board;
class MoveGenerator;
class Evaluator;
class Notator;

class Game
{
public:
   // =========================================================================
   // Destruction
   // =========================================================================
   virtual ~Game();
   
   // =========================================================================
   // Operations
   // =========================================================================
   bool IsPieceOnSquare(int square);
   bool IsPieceColor(int square, int side);
   std::string GetPieceName(int piecetype, int language, bool promoted);

   virtual bool IsLegal(Move* move, int side) = 0;
   virtual bool IsValid(Move* move, int side) { return true; };
   virtual bool CanPromote(Move* move) = 0;
   virtual bool CanMoveTwice(int start, int finish) { return false; }
   virtual bool GameWon(int side) = 0;
   virtual bool GameDrawn() = 0;

   virtual bool AllowDrops() { return false; }

   virtual void PostProcess(std::vector<Move>& movelist, int side) {};
   
   void MakeMove(Move* move);
   void UnmakeMove(Move* move);

   virtual int  GetPromotedType(int piecetype) = 0;

   virtual bool ForcePromotion(const Move& move) = 0;
   
   Move* ParseMove(const std::string& movestring);
   std::string Notate(Move* move);
   std::string NotateBoard();
   
   virtual void Reset() = 0;

   virtual Move* CreateMove(const MoveRec& moverec) = 0;
   
   void GetPieceInfo(std::vector<PieceInfo>& squares);
   void GetDestinationSquares(int index, std::vector<int>& squares);
   virtual void GetSecondSquares(int start, int middle, 
                                std::vector<int>& squares);
   
   // =========================================================================
	// Constants
	// =========================================================================
  
   enum {
      IN_PROGRESS = 0,
      BLACK_WIN,
      WHITE_WIN,
      DRAW
   };
   
   // =========================================================================
	// Data
	// =========================================================================
   Board*         mpBoard;          // Represents the board position
   MoveGenerator* mpMoveGenerator;  // Generates moves from a board position
   Evaluator*     mpEvaluator;      // Evaluates a board position
   Notator*       mpNotator;        // Reads and writes move notation

   int            mStatus;          // Status of the game
   bool           mAllowDrops;      // Indicates whether drops are allowed
                                    // in this game
   
   std::vector<PieceType*> mPieceTypes;  // Piece types in this game
   std::vector<Piece> mPieces[2];        // Pieces in the game

   std::vector<uint64> mHashKeys;        // Hash keys for every board state
                                         // that has lead to the current state
};

#endif // GAME_HPP
