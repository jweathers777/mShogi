////////////////////////////////////////////////////////////////////////////
// Name:         PVSearch.cpp
// Description:  Implementation for class that performs a
//               principle variation search
// Created:      07/09/2004 04:34:22 Eastern Standard Time
// Last Updated: $Date: 2004/09/18 21:59:43 $
// Revision:     $Revision: 1.3 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// STL header files
#include <vector>
#include <string>
#include <iostream>
#include <sstream>

// Local header files
#include "common.hpp"
#include "PVSearch.hpp"
#include "Engine.hpp"
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
        << setw(10) << mpNotator->Notate(move) \
        << " [" << setw(9) << alpha \
        << ","  << setw(9) << beta << "] " << name << endl; \
} \

#define PRINT_SCORE(depth, score, alpha, beta) \
if (keeplog) { \
   gLog << setw(2*depth) << " " \
        << (depth+1) << " SCORE =" << setw(10) << score \
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
//       Class:  PVSearch
//      Method:  GetBestMove
// Description:  Return the best move available for the given side as
//               determined by the search algorithm
//--------------------------------------------------------------------------
Move* PVSearch::GetBestMove(int side) 
{
   bool winningMove = false;
   bool foundPV = false;

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
         if (foundPV) {
            score = -DoSearch(1, -alpha-1, -alpha, line);
            if ((score > alpha) && (score < beta)) {
               // Oops. This wasn't really a Principle Variation node
               score = -DoSearch(1, -beta, -alpha, line);
            }
         }
         else {
            score = -DoSearch(1, -beta, -alpha, line);
         }
         if (mAbort) break;
         mpRepTable->RemoveKey( mpBoard->UnmakeMove(move) );
         PRINT_SCORE(0, score, alpha, beta);
         
         if (score > alpha) { // Is this score good enough?
            bestmove->Copy(*move);
            alpha = score;

            foundPV = true;
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

   // Calculate the time difference in seconds
   DIFF_PRECISION_TIME(mSeconds, finishTime, startTime); 
   
   return bestmove;
}

//--------------------------------------------------------------------------
//       Class:  PVSearch
//      Method:  DoSearch
// Description:  Performs the actual principle variation search
//--------------------------------------------------------------------------
int PVSearch::DoSearch(int depth, int alpha, int beta, vector<Move>& pline)
{
   bool foundPV = false;
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
      // Make sure that our opponent doesn't win on the next move
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
            if (foundPV) {
               score = -DoSearch(depth+1, -alpha-1, -alpha, line);
               if ((score > alpha) && (score < beta)) {
                  // Oops. This wasn't really a Principle Variation node
                  score = -DoSearch(depth+1, -beta, -alpha, line);
               }
            }
            else {
               score = -DoSearch(depth+1, -beta, -alpha, line);
            }
         }
         if (mAbort) return 0;
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
            foundPV = true;
            
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

