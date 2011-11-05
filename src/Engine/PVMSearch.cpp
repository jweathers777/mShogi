////////////////////////////////////////////////////////////////////////////
// Name:         PVMSearch.cpp
// Description:  Implementation for class that performs a memory enhanced
//               principle variation search
// Created:      07/28/2004 10:00:25 Eastern Daylight Time
// Last Updated: $Date: 2004/09/18 21:59:43 $
// Revision:     $Revision: 1.3 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// Standard library files
#include <cstring>

// STL header files
#include <vector>
#include <string>
#include <iostream>
#include <sstream>

// Local header files
#include "common.hpp"
#include "PVMSearch.hpp"
#include "Engine.hpp"
#include "Game.hpp"
#include "Board.hpp"
#include "MoveGenerator.hpp"
#include "Evaluator.hpp"
#include "Notator.hpp"
#include "Move.hpp"
#include "MoveRec.hpp"
#include "ByValue.hpp"
#include "RepetitionTable.hpp"
#include "TranspositionTable.hpp"

using std::vector;
using std::string;
using std::ostringstream;
using std::endl;

#ifdef DEBUG

#include <fstream>
#include <iomanip>

using std::setw;
using std::ostream;

extern std::ofstream gLog;

// Overload output operator for MoveRec structures
extern ostream& operator <<(ostream& output, const MoveRec& moverec);

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

#define PRINT_HASH_MOVE(moveptr, moverec) \
if (keeplog) { \
   if (moveptr) { \
      gLog << "Hash Move: " << mpNotator->Notate(moveptr) << endl; \
   }\
   else { \
      gLog << "WARNING******************************************" << endl; \
      gLog << moverec << "is not a valid move for this position." << endl; \
      gLog << "WARNING******************************************" << endl; \
   } \
}

#else
#define PRINT_DESCENT(depth, alpha, beta, move, name)
#define PRINT_SCORE(depth, score, alpha, beta)
#define LINE_REACHED(value)
#define PRINT_HASH_MOVE(moveptr, moverec)
#endif

//--------------------------------------------------------------------------
//       Class:  PVMSearch
//      Method:  GetBestMove
// Description:  Return the best move available for the given side as
//               determined by the search algorithm
//--------------------------------------------------------------------------
Move* PVMSearch::GetBestMove(int side) 
{
   bool winningMove = false;
   bool foundPV = false;

   int alpha = mAlpha;
   int beta =  mBeta;
   int score; 
   PRECISION_TIME startTime, finishTime;
   
   vector<Move> movelist;
   vector<Move> line;

   Move* bestmove = 0;
   Move* move = 0;
   
   // Increase the time stamp indicator
   TranspositionTable::sTimeStamp++;
   
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
      
      mPVScore = MINIMUM_SCORE;
      
      // Initialize our optimal move, line, and score
      if (!movelist.empty()) {
         bestmove = new Move( movelist.back() );
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
            if ((score > alpha) && (score < beta)) { // Check for failure
               score = -DoSearch(1, -beta, -alpha, line);
            }
         }
         else {
            score = -DoSearch(1, -beta, -alpha, line);
         }
         if (mAbort) break;
         if (score > mPVScore) {
            bestmove->Copy(*move);
            mPVScore = score;
            
            mPrincipleVariation.clear();
            mPrincipleVariation.push_back(*move);
            mPrincipleVariation.insert(mPrincipleVariation.end(), 
                                       line.begin(), line.end());
         }
            
         mpRepTable->RemoveKey( mpBoard->UnmakeMove(move) );
         PRINT_SCORE(0, score, alpha, beta);
         
         if (score > alpha) {
            alpha = score;
            foundPV = true;
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
int PVMSearch::DoSearch(int depth, int alpha, int beta, vector<Move>& pline)
{
   bool foundPV = false;
   char flags = TranspositionTable::ALPHA_FLAG;
   int score;
   MoveRec tableMoveRec = gNullMoveRec;
   vector<Move> line;
   int remply = mSearchDepth - depth;

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
      if (score != MAXIMUM_SCORE ) {
         score = REPETITION_SCORE;
      }
   }
   // Check the transposition table for cutoffs and good moves
   else if (mpTranTable[mSide]->ProbeTable(remply, alpha, beta, 
                                           mpBoard->mHashKey,
                                           score, tableMoveRec)) {
      // We found a cutoff and have a score to return
   }
   // Check for whether we have reached the end of our search depth
   else if (depth == mSearchDepth) {
      if (mQuiescenceOn) 
         score = DoQuiescenceSearch(1, alpha, beta, line);
      else {
         score = mpEvaluator->FullEvaluation(mSide);
         mpTranTable[mSide]->StoreEntry(remply, score, mpBoard->mHashKey,
                                        TranspositionTable::EXACT_FLAG,
                                        gNullMoveRec);
      }
   }
   // Conduct an actual search
   else {
      int phase = MoveGenerator::GENERATE_CAPTURES;
      vector<Move> movelist;
      Move* tableMovePtr = 0;

      // Do we have a transposition table record for this position?
      if (tableMoveRec.mPiecePtr != 0) {
         // Convert our move record to a new move object
         tableMovePtr = mpGame->CreateMove(tableMoveRec);
         PRINT_HASH_MOVE(tableMovePtr, tableMoveRec);

         // Set the phase so we process this move before the others
         phase = MoveGenerator::PROCESS_HASH_MOVE;
      }

      // Start generating moves
      Move* move = mpMoveGenerator->GetNextMove(movelist, phase, mSide,
                                                tableMovePtr);
      
      // Re-initialize tableMoveRec for storing the best move found
      memcpy(&tableMoveRec, &gNullMoveRec, sizeof(MoveRec));
      
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
               if ((score > alpha) && (score < beta)) { // Check for failure
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
            
            // Fill the move record with this move
            move->FillMoveRec(tableMoveRec);
            flags = TranspositionTable::BETA_FLAG;

            move = 0;
         }
         else if (score > alpha) { // Is this score good enough to use?
            alpha = score;
            pline.clear();
            pline.push_back(*move);
            pline.insert(pline.end(), line.begin(), line.end());
            foundPV = true;

            // Fill the move record with this move as the current best
            // move for storing in the transposition table
            move->FillMoveRec(tableMoveRec);
            flags = TranspositionTable::EXACT_FLAG;

            // Get the next move for processing            
            movelist.pop_back();
            move = mpMoveGenerator->GetNextMove(movelist, phase, mSide);
            if (tableMovePtr && (*move == *tableMovePtr)) {
               delete tableMovePtr;
               tableMovePtr = 0;

               movelist.pop_back();
               move = mpMoveGenerator->GetNextMove(movelist, phase, mSide);
            }
         }
         else {
            // Get the next move for processing
            movelist.pop_back();
            move = mpMoveGenerator->GetNextMove(movelist, phase, mSide);
            if (tableMovePtr && (*move == *tableMovePtr)) {
               delete tableMovePtr;
               tableMovePtr = 0;
               
               movelist.pop_back();
               move = mpMoveGenerator->GetNextMove(movelist, phase, mSide);
            }
         }
      }
      
      // Clean up memory for pointer to transposition table move if necessary
      if (tableMovePtr) {
         delete tableMovePtr;
         tableMovePtr = 0;
      }

      // Store an entry in the transposition table
      mpTranTable[mSide]->StoreEntry(remply, alpha, mpBoard->mHashKey,
                                     flags, tableMoveRec);
      
      // Set the score that we will return
      score = alpha;
   }

   // Flip sides back 
   mSide = 1 - mSide;
   
   return score;
}

