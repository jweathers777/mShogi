////////////////////////////////////////////////////////////////////////////////
// Name:         AlphaBetaMemory
// Description:  Interface for class that performs memory enhanced
//               alpha beta search
// Created:      Jul 14, 2004 08:42:22 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef ALPHABETAMEMORY_HPP
#define ALPHABETAMEMORY_HPP

#include <vector>
#include "Search.hpp"

// Forward declarations
class Game;
class Board;
class MoveGenerator;
class Evaluator;
class Notator;
class RepetitionTable;
class TranspositionTable;

class AlphaBetaMemory : public Search
{
public:
   // ======================================================================
   // Constructors and Destructors
   // ======================================================================
	AlphaBetaMemory();
	virtual ~AlphaBetaMemory();
   
   // ======================================================================
   // Accessors
   // ======================================================================
   void SetEngine(Engine* engine);
   void SetGame(Game* game);
   void SetHashSize(int size);

   // ======================================================================
   // Operations
   // ======================================================================
   virtual Move* GetBestMove(int side);
   std::string GetPVString(const std::vector<Move>& line);
   std::string GetStatistics();
   void Reset() { return; }
   
protected:
   void PrepareSearch(bool clearPV=true);
   void SortRootChildren(bool& winningMove,
                         std::vector<Move>& movelist);
   
   virtual int DoSearch(int depth, int alpha, int beta, 
                        std::vector<Move>& pline);
   virtual int DoQuiescenceSearch(int depth, int alpha, int beta,
                                  std::vector<Move>& pline);
   
   // ======================================================================
   // Constants
   // ======================================================================
   enum { 
      REP_TABLE_SIZE = 2048,
      // TRAN_TABLE_SIZE = 4096
      // TRAN_TABLE_SIZE = 8192
      // TRAN_TABLE_SIZE = 16384
      // TRAN_TABLE_SIZE = 32768
      // TRAN_TABLE_SIZE = 65536
      TRAN_TABLE_SIZE = 131072
   };
      
   // ======================================================================
   // Data
   // ======================================================================
   Board*         mpBoard;          // Represents the board state
   MoveGenerator* mpMoveGenerator;  // Generates moves from the board state
   Evaluator*     mpEvaluator;      // Evaluates the board state
   Notator*       mpNotator;        // Notates the board state
   Game*          mpGame;           // Represents the game
   Engine*        mpEngine;         // Pointer to the game engine

   // Hash table for detecting repetitions
   RepetitionTable* mpRepTable;
   
   // Hash tables for detecting transpositions
   TranspositionTable* mpTranTable[2];

   int mTranTableSize; // Size of the transposition tables
   
   int  mSide;  // Current side in the search

   int  mNextCheck; // Number of nodes left to search before checking
                    // whether we should abort the search
   bool mAbort;     // Flag that indicates whether this search 
                    // should abort

   std::vector<Move> mPrincipleVariation; // Principle Variation line
   int mPVScore;     // Believed principle variation score

   int mAlpha, mBeta;  // Initial alpha and beta for root search

   // ======================================================================
   // Statistics Data
   // ======================================================================
   double mSeconds;       // Time of the most recent search in seconds
      
   int mNodes;            // Total nodes searched
   int mCutOffs;          // Total cut-offs produced
   int mQuiescentNodes;   // Total quiescent nodes searched
   int mQuiescentCutOffs; // Total quiescent cut-offs produced
};

#endif // ALPHABETAMEMORY_HPP
