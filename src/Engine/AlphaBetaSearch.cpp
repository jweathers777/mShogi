////////////////////////////////////////////////////////////////////////////////
// Name:         AlphaBetaSearch
// Description:  Implementation for class that performs an alpha beta search
// Created:      Apr 20, 2004 11:20:49 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// STL header files
#include <vector>
#include <string>
#include <iostream>
#include <sstream>

// Local header files
#include "common.hpp"
#include "Engine.hpp"
#include "AlphaBetaSearch.hpp"
#include "Game.hpp"
#include "Board.hpp"
#include "MoveGenerator.hpp"
#include "Evaluator.hpp"
#include "Notator.hpp"
#include "Move.hpp"
#include "ByValue.hpp"
#include "RepetitionTable.hpp"

using std::vector;
using std::string;
using std::ostringstream;
using std::endl;

#ifdef DEBUG

#include <fstream>
#include <iomanip>

using std::setw;

extern std::ofstream gLog;

#define PRINT_DESCENT(depth, alpha, beta, move, name) \
if (keeplog) { \
   gLog << setw(2*depth) << " " \
        << (depth+1) \
        << setw(12) << mpNotator->Notate(move) \
        << " [" << setw(9) << alpha \
        << ","  << setw(9) << beta << "] " << name << endl; \
} \

#define PRINT_SCORE(depth, score, alpha, beta) \
if (keeplog) { \
   gLog << setw(2*depth) << " " \
        << (depth+1) << " SCORE =" << setw(12) << score \
        << " [" << setw(9) << alpha \
        << ","  << setw(9) << beta << "] " << endl; \
} \

#define LINE_REACHED(value) \
if (keeplog) { \
   gLog << "Line #" << value << " Reached" << endl; \
}

#else
#define PRINT_DESCENT(depth, alpha, beta, move, name)
#define PRINT_SCORE(depth, score, alpha, beta)
#define LINE_REACHED(value)
#endif

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  AlphaBetaSearch
// Description:  Constructs an instance of an alpha beta search
//--------------------------------------------------------------------------
AlphaBetaSearch::AlphaBetaSearch()
{
   // Initialize game related object pointers to null
   // These must be set externally before using the search
   mpEngine = 0;
   mpGame = 0;
   mpBoard = 0;
   mpMoveGenerator = 0;
   mpEvaluator = 0;
   mpNotator = 0;

   mpRepTable = new RepetitionTable(REP_TABLE_SIZE);
   
   // Initialize search parameters
   mSearchTime = DEFAULT_SEARCH_TIME;
   mSearchDepth = DEFAULT_SEARCH_DEPTH;
   mSearchNodes = DEFAULT_SEARCH_NODES;

   mQuiescenceOn = false;
   mQuiescenceDepth = 2;

   mSide = 0;

   mNextCheck = NODES_BETWEEN_TIME_CHECKS;
   mAbort = false;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  ~AlphaBetaSearch
// Description:  Destroys this instance of an alpha beta search
//--------------------------------------------------------------------------
AlphaBetaSearch::~AlphaBetaSearch()
{
   if (mpRepTable) {
      delete mpRepTable;
      mpRepTable = 0;
   }
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  SetEngine
// Description:  Sets the engine object that owns this search
//--------------------------------------------------------------------------
void AlphaBetaSearch::SetEngine(Engine* engine)
{
   mpEngine = engine;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  SetGame
// Description:  Sets the game object and all dependent objects
//--------------------------------------------------------------------------
void AlphaBetaSearch::SetGame(Game* game)
{
   mpGame = game;
   mpBoard = game->mpBoard;
   mpMoveGenerator = game->mpMoveGenerator;
   mpEvaluator = game->mpEvaluator;
   mpNotator = game->mpNotator;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  GetBestMove
// Description:  Return the best move available for the given side as
//               determined by the search algorithm
//--------------------------------------------------------------------------
Move* AlphaBetaSearch::GetBestMove(int side) 
{
   bool winningMove = false;
   int alpha = MINIMUM_SCORE;
   int beta =  MAXIMUM_SCORE;
   int score;
   PRECISION_TIME startTime, finishTime;
   
   vector<Move> movelist;
   vector<Move> line;

   Move* bestmove = 0;
   Move* move = 0;
   
   // Get the start time for the search
   GET_PRECISION_TIME(startTime);
   
   // For which side are we searching?
   mSide = side;

   // Generate all legal moves for the first ply
   mpMoveGenerator->GenerateMoves(movelist, side);

   // Perform a special move sort for the root of the search
   SortRootChildren(winningMove, movelist);

   // We're done if we found a winning move during the sort
   if (winningMove) {
      mPVScore = MAXIMUM_SCORE;
      mPrincipleVariation.push_back( movelist.back() );
      bestmove = new Move( movelist.back() );
   }
   else {
      // Prep the search
      PrepareSearch();
      
      // Clear out the line
      line.clear();
      
      // Initialize our optimal move, line, and score
      if (!movelist.empty()) {
         bestmove = new Move( movelist.back() );
         mPVScore = bestmove->mValue;
         mPrincipleVariation.push_back(*bestmove);
      }
      
      // Initialize values for monitoring whether we need to stop
      mNextCheck = NODES_BETWEEN_TIME_CHECKS;
      mAbort = false;

      // Recursively search each move
      do {
         move = &(movelist.back());
         mNodes++; // Count the nodes that we've search

         mpRepTable->AddKey( mpBoard->MakeMove(move) );
         PRINT_DESCENT(0, alpha, beta, move, "Root");
         score = -DoSearch(1, -beta, -alpha, line);
         if (mAbort) break;
         mpRepTable->RemoveKey( mpBoard->UnmakeMove(move) );
         PRINT_SCORE(0, score, alpha, beta);
         
         if (score > alpha) { // Is this score good enough?
            bestmove->Copy(*move);
            alpha = score;

            mPVScore = alpha;
            
            mPrincipleVariation.clear();
            mPrincipleVariation.push_back(*move);
            mPrincipleVariation.insert(mPrincipleVariation.end(), 
                                       line.begin(), line.end());
         }
         
         movelist.pop_back();
      } while( !movelist.empty() );
   }
   
   // Get the finish time for the search
   GET_PRECISION_TIME(finishTime);

   // Calculate the time difference in fractional seconds
   DIFF_PRECISION_TIME(mSeconds, finishTime, startTime); 
   
   return bestmove;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  GetPVString
// Description:  Return a principle variation string
//--------------------------------------------------------------------------
string AlphaBetaSearch::GetPVString(const vector<Move>& line)
{
   ostringstream output;

   for (unsigned int i = 0; i < line.size(); i++) {
      output << (i+1) << ". " << mpNotator->Notate(&(line[i])) << " ";
   }
   
   return output.str();
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  GetStatistics
// Description:  Return a string with a report about search statistics
//--------------------------------------------------------------------------
string AlphaBetaSearch::GetStatistics() 
{
   ostringstream output;

   output << "+-----------------+" << endl;
   output << "|Search Statistics|" << endl;
   output << "+-----------------+" << endl;
   output << "Nodes Searched   = " << mNodes << endl;
   output << "Branches Cut Off = " << mCutOffs << endl;
   output << "Quiescent Nodes Searched   = " << mQuiescentNodes << endl;
   output << "Quiescent Branches Cut Off = " << mQuiescentCutOffs << endl;
   output << "+-------------------+" << endl;
   output << "|Principle Variation|" << endl;
   output << "+-------------------+" << endl;
   output << GetPVString(mPrincipleVariation) << endl;
   output << "Score Estimate = " << mPVScore << endl;
   output << "Search Time    = " << mSeconds << " seconds" << endl;

   return string(output.str());
}

//-------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  PrepareSearch
// Description:  Prepare for a fresh search
//-------------------------------------------------------------------------
void AlphaBetaSearch::PrepareSearch(bool clearPV)
{
   unsigned int i;

   // Initialize PV score
   mPVScore = MINIMUM_SCORE;

   // Clear out the principle variation vector
   if (clearPV) mPrincipleVariation.clear();
   
   // Initialize statistics
   mNodes = 0;
   mCutOffs = 0;
   mQuiescentNodes = 0;
   mQuiescentCutOffs = 0;

   // Clear the Repetition Table
   mpRepTable->ClearTable();

   // Initialize Repetition Table
   for (i = 0; i < mpGame->mHashKeys.size(); i++) {
      mpRepTable->AddKey(mpGame->mHashKeys[i]);
   } 
}

//-------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  SortRootChildren
// Description:  Special sorting for the moves generated at the search root
//-------------------------------------------------------------------------
void AlphaBetaSearch::SortRootChildren(bool& winningMove, 
                                       vector<Move>& movelist)
{
   unsigned int i;
   int score;
   vector<Move> line;
   vector<int> values(movelist.size());

   winningMove = false;
   
   // Prep for a search
   PrepareSearch(false);
   
   // Conduct a small quiescence search on each move
   // and use the scores as sorting values
   for (i = 0; i < movelist.size(); i++) {
      // Store original move value
      values[i] = movelist[i].mValue;
      
      // Inflate the score for this move if it was our
      // next move from the prior Principle Variation line
      if ((mPrincipleVariation.size() > 1) &&
          (mPrincipleVariation[1] == movelist[i])) {
         movelist[i].mValue = MAXIMUM_SCORE-1;
      }
      else {
         mpBoard->MakeMove(&(movelist[i]));
         // Does this move win the game?
         if ( mpGame->GameWon(mSide) ) {
            mpBoard->UnmakeMove(&(movelist[i]));            
            winningMove = true;
            Move bestmove( movelist[i] );
            movelist.clear();
            movelist.push_back(bestmove);
            break;
         }
         // Is this move one that results in a draw?
         else if (mpGame->GameDrawn()) {
            score = DRAW_SCORE;
         }
         // Search this move for a score estimate
         else {
            mSide = 1-mSide;
            PRINT_DESCENT(mSearchDepth-1, MINIMUM_SCORE, MAXIMUM_SCORE, 
                          &(movelist[i]), "Pre-Root");
            score = -DoQuiescenceSearch(mQuiescenceDepth-1, 
                                        MINIMUM_SCORE, MAXIMUM_SCORE, line);
            PRINT_SCORE(mSearchDepth-1, score, MINIMUM_SCORE, MAXIMUM_SCORE);
            mSide = 1-mSide;
         }
         mpBoard->UnmakeMove(&(movelist[i]));
         movelist[i].mValue = score;
      }
   }

   if (!winningMove) {
      // Sort the moves by their scores
      sort( movelist.begin(), movelist.end(), ByValue<Move>() );

      // Reset the original move values
      for (i = 0; i < movelist.size(); i++) movelist[i].mValue = values[i];
   }
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  DoSearch
// Description:  Performs the actual alpha beta search
//--------------------------------------------------------------------------
int AlphaBetaSearch::DoSearch(int depth, int alpha, int beta, 
                              vector<Move>& pline)
{
   int score;
   vector<Move> line;

   // Check to see whether we should abort
   if (--mNextCheck <= 0) {
      mNextCheck = NODES_BETWEEN_TIME_CHECKS;
      if ( mpEngine->SearchAborted() ) {
         mAbort = true;
         return 0;
      }
   }

   // Flip sides
   mSide = 1 - mSide;
   
   // Check for any search repetitions
   if ( mpRepTable->CheckForRepetition(mpBoard->mHashKey) ) {
      // Make sure that there isn't a win on the next move
      score = DoQuiescenceSearch(mQuiescenceDepth-1, MINIMUM_SCORE, 
                                 MAXIMUM_SCORE, line);
      if (score != MAXIMUM_SCORE) {
         score = REPETITION_SCORE;
      }
   }
   // Check for whether we have reached the end of our search depth
   else if (depth == mSearchDepth) {
      if (mQuiescenceOn) 
         score = DoQuiescenceSearch(1, alpha, beta, line);
      else
         score = mpEvaluator->FullEvaluation(mSide);
   }
   // Conduct an actual search
   else {
      int phase = MoveGenerator::GENERATE_CAPTURES;
      vector<Move> movelist;

      // Start generating moves
      Move* move = mpMoveGenerator->GetNextMove(movelist, phase, mSide);
      
      // Recursively search each move
      while(move) {
         mNodes++; // Count the nodes that we search
         
         mpRepTable->AddKey( mpBoard->MakeMove(move) );
         PRINT_DESCENT(depth, alpha, beta, move, "Search");
         if ( mpGame->GameWon(mSide) ) {
            score = MAXIMUM_SCORE;
         }
         else {
            score = -DoSearch(depth+1, -beta, -alpha, line);
            if (mAbort) return 0;
         }
         mpRepTable->RemoveKey( mpBoard->UnmakeMove(move) );
         PRINT_SCORE(depth, score, alpha, beta);
         
         if (score >= beta) { // Check for a beta cutoff
            mCutOffs++;
            alpha = beta;
            move = 0;
         }
         else if (score > alpha) { // Is this score good enough to use?
            alpha = score;
            pline.clear();
            pline.push_back(*move);
            pline.insert(pline.end(), line.begin(), line.end());
            
            movelist.pop_back();
            move = mpMoveGenerator->GetNextMove(movelist, phase, mSide);
         }
         else {
            movelist.pop_back();
            move = mpMoveGenerator->GetNextMove(movelist, phase, mSide);
         }
      }
      
      // Set the score that we will return
      score = alpha;
   }

   // Flip sides back 
   mSide = 1 - mSide;
   
   return score;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaSearch
//      Method:  DoQuiescenceSearch
// Description:  Performs the quiescence search
//--------------------------------------------------------------------------
int AlphaBetaSearch::DoQuiescenceSearch(int depth, int alpha, int beta,
                                        vector<Move>& pline)
{
   int score = mpEvaluator->FullEvaluation(mSide);

   if (depth < mQuiescenceDepth) {
      int totaldepth = depth + mSearchDepth - 1;
      int phase = MoveGenerator::GENERATE_CAPTURES;
      vector<Move> movelist;
      vector<Move> line;

      Move* move = mpMoveGenerator->GetNextCapture(movelist, phase, mSide);
      if (move) {
         do {
            mQuiescentNodes++; // Count the nodes that we search
            
            mpBoard->MakeMove(move);
            PRINT_DESCENT(totaldepth, alpha, beta, move, "Quiescence");
            if ( mpGame->GameWon(mSide) ) {
               score = MAXIMUM_SCORE;
            }
            else {
               mSide=1-mSide;
               score = -DoQuiescenceSearch(depth+1, -beta, -alpha, line);
               mSide=1-mSide;
            }
            mpBoard->UnmakeMove(move);
            PRINT_SCORE(totaldepth, score, alpha, beta);

            if (score >= beta) { // Check for a beta cutoff
               mQuiescentCutOffs++;
               alpha = beta;
               move = 0;
            }
            else if (score > alpha) { // Is this score good enough to use?
               alpha = score;
               pline.clear();
               pline.push_back(*move);
               pline.insert(pline.end(), line.begin(), line.end());

               movelist.pop_back();
               move = mpMoveGenerator->GetNextCapture(movelist, phase, mSide);
            }
            else {
               movelist.pop_back();
               move = mpMoveGenerator->GetNextCapture(movelist, phase, mSide);
            }
         } while (move);
         
         score = alpha;
      }
   }

   return score;
}
