////////////////////////////////////////////////////////////////////////////////
// Name:         AlphaBetaSearch
// Description:  Interface for class that performs an alpha beta search
// Created:      Apr 20, 2004 11:09:53 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef ALPHABETASEARCH_HPP
#define ALPHABETASEARCH_HPP

#include <vector>
#include "Search.hpp"

// Forward declarations
class Game;
class Board;
class MoveGenerator;
class Evaluator;
class Notator;
class RepetitionTable;

class AlphaBetaSearch : public Search
{
public:
   // ======================================================================
   // Constructors and Destructors
   // ======================================================================
	AlphaBetaSearch();
	virtual ~AlphaBetaSearch();
   
   // ======================================================================
   // Accessors
   // ======================================================================
   void SetEngine(Engine* engine);
   void SetGame(Game* game);
   void SetHashSize(int size) { return; }

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
   int DoQuiescenceSearch(int depth, int alpha, int beta,
                          std::vector<Move>& pline);
   
   // ======================================================================
   // Constants
   // ======================================================================
   enum { REP_TABLE_SIZE = 2048 };
      
   // ======================================================================
   // Data
   // ======================================================================
   Board*         mpBoard;          // Represents the board state
   MoveGenerator* mpMoveGenerator;  // Generates moves from the board state
   Evaluator*     mpEvaluator;      // Evaluates the board state
   Notator*       mpNotator;        // Notates the board state
   Game*          mpGame;           // Represents the game
   Engine*        mpEngine;         // Pointer to the game engine

   RepetitionTable* mpRepTable;     // Hash table for detecting repetitions
   
   int            mSide;            // Current side in the search

   int  mNextCheck; // Number of nodes left to search before checking
                    // whether we should abort the search
   bool mAbort;     // Flag that indicates whether this search 
                    // should abort

   std::vector<Move> mPrincipleVariation; // Principle Variation line
   int mPVScore;     // Believed principle variation score

   // ======================================================================
   // Statistics Data
   // ======================================================================
   double mSeconds;       // Time of the most recent search in seconds
      
   int mNodes;            // Total nodes searched
   int mCutOffs;          // Total cut-offs produced
   int mQuiescentNodes;   // Total quiescent nodes searched
   int mQuiescentCutOffs; // Total quiescent cut-offs produced
};

#endif // ALPHABETASEARCH_HPP
