////////////////////////////////////////////////////////////////////////////////
// Name:         AlphaBetaMemory
// Description:  Implementation for class that performs an alpha beta search
// Created:      Jul 14, 2004 08:46:37 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// Standard library files
#include <cstring>

// STL header files
#include <vector>
#include <string>
#include <iostream>
#include <sstream>

// Local header files
#include "common.hpp"
#include "Engine.hpp"
#include "AlphaBetaMemory.hpp"
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
//       Class:  AlphaBetaMemory
//      Method:  AlphaBetaMemory
// Description:  Constructs an instance of a memory
//               enhanced alpha beta search
//--------------------------------------------------------------------------
AlphaBetaMemory::AlphaBetaMemory()
{
   // Initialize game related object pointers to null
   // These must be set externally before using the search
   mpEngine = 0;
   mpGame = 0;
   mpBoard = 0;
   mpMoveGenerator = 0;
   mpEvaluator = 0;
   mpNotator = 0;

   mTranTableSize = TRAN_TABLE_SIZE;

   mpRepTable = new RepetitionTable(REP_TABLE_SIZE);
   mpTranTable[BLACK] = new TranspositionTable(mTranTableSize);
   mpTranTable[WHITE] = new TranspositionTable(mTranTableSize);

   // Initialize time stamp indicator
   TranspositionTable::sTimeStamp = 0;

   // Initialize root search alpha and beta inital values
   mAlpha = MINIMUM_SCORE;
   mBeta = MAXIMUM_SCORE;
   
   // Initialize search parameters
   mSearchTime = DEFAULT_SEARCH_TIME;
   mSearchDepth = DEFAULT_SEARCH_DEPTH;
   mSearchNodes = DEFAULT_SEARCH_NODES;

   mQuiescenceOn = true;
   mQuiescenceDepth = 2;

   mSide = 0;

   mNextCheck = NODES_BETWEEN_TIME_CHECKS;
   mAbort = false;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  ~AlphaBetaMemory
// Description:  Destroys this instance of an alpha beta search
//--------------------------------------------------------------------------
AlphaBetaMemory::~AlphaBetaMemory()
{
   if (mpRepTable) {
      delete mpRepTable;
      mpRepTable = 0;
   }
   if (mpTranTable[BLACK]) {
      delete mpTranTable[BLACK];
      mpTranTable[BLACK] = 0;
   }
   if (mpTranTable[WHITE]) {
      delete mpTranTable[WHITE];
      mpTranTable[WHITE] = 0;
   }
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  SetEngine
// Description:  Sets the engine object that owns this search
//--------------------------------------------------------------------------
void AlphaBetaMemory::SetEngine(Engine* engine)
{
   mpEngine = engine;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  SetGame
// Description:  Sets the game object and all dependent objects
//--------------------------------------------------------------------------
void AlphaBetaMemory::SetGame(Game* game)
{
   mpGame = game;
   mpBoard = game->mpBoard;
   mpMoveGenerator = game->mpMoveGenerator;
   mpEvaluator = game->mpEvaluator;
   mpNotator = game->mpNotator;

   // Clear the transposition tables of their junk contents
   mpTranTable[BLACK]->ClearTable();
   mpTranTable[WHITE]->ClearTable();
   
   // Reset the timestamp
   TranspositionTable::sTimeStamp = 0;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  SetHashSize
// Description:  Sets the hash size for the transposition tables
//--------------------------------------------------------------------------
void AlphaBetaMemory::SetHashSize(int size)
{
   mTranTableSize = size;

   // Delete the transposition tables 
   if (mpTranTable[BLACK]) {
      delete mpTranTable[BLACK];
   };
   if (mpTranTable[WHITE]) {
      delete mpTranTable[WHITE];
   };
   
   // Create new tables with the new table size
   mpTranTable[BLACK] = new TranspositionTable(mTranTableSize);
   mpTranTable[WHITE] = new TranspositionTable(mTranTableSize);
   
   // Reset the timestamp
   TranspositionTable::sTimeStamp = 0;
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  GetBestMove
// Description:  Return the best move available for the given side as
//               determined by the search algorithm
//--------------------------------------------------------------------------
Move* AlphaBetaMemory::GetBestMove(int side) 
{
   bool winningMove = false;
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
         if (!move->mValidate || mpGame->IsValid(move, side)) {
            mNodes++; // Count the nodes that we've search

            mpRepTable->AddKey( mpBoard->MakeMove(move) );
            PRINT_DESCENT(0, alpha, beta, move, "Root");
            score = -DoSearch(1, -beta, -alpha, line);
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
            if (mAbort) break;
            
            if (score > alpha) { // Update alpha
               alpha = score;
            }
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
//       Class:  AlphaBetaMemory
//      Method:  GetPVString
// Description:  Return a principle variation string
//--------------------------------------------------------------------------
string AlphaBetaMemory::GetPVString(const vector<Move>& line)
{
   ostringstream output;

   for (unsigned int i = 0; i < line.size(); i++) {
      output << (i+1) << ". " << mpNotator->Notate(&(line[i])) << " ";
   }
   
   return output.str();
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  GetStatistics
// Description:  Return a string with a report about search statistics
//--------------------------------------------------------------------------
string AlphaBetaMemory::GetStatistics() 
{
   ostringstream output;

   output << "+-----------------+" << endl;
   output << "|Search Statistics|" << endl;
   output << "+-----------------+" << endl;
   output << "Nodes Searched   = " << mNodes << endl;
   output << "Branches Cut Off = " << mCutOffs << endl;
   output << "Quiescent Nodes Searched   = " << mQuiescentNodes << endl;
   output << "Quiescent Branches Cut Off = " << mQuiescentCutOffs << endl;
   output << "+---------------------------------------+" << endl;
   output << "|Friendly Transposition Table Statistics|" << endl;
   output << "+---------------------------------------+" << endl;
   output << mpTranTable[mSide]->GetStatistics();
   output << "+------------------------------------+" << endl;
   output << "|Enemy Transposition Table Statistics|" << endl;
   output << "+------------------------------------+" << endl;
   output << mpTranTable[1-mSide]->GetStatistics();
   output << "+-------------------+" << endl;
   output << "|Principle Variation|" << endl;
   output << "+-------------------+" << endl;
   output << GetPVString(mPrincipleVariation) << endl;
   output << "Score Estimate = " << mPVScore << endl;
   output << "Search Time    = " << mSeconds << " seconds" << endl;

   return string(output.str());
}

//-------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  PrepareSearch
// Description:  Prepare for a fresh search
//-------------------------------------------------------------------------
void AlphaBetaMemory::PrepareSearch(bool clearPV)
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
//       Class:  AlphaBetaMemory
//      Method:  SortRootChildren
// Description:  Special sorting for the moves generated at the search root
//-------------------------------------------------------------------------
void AlphaBetaMemory::SortRootChildren(bool& winningMove, 
                                       vector<Move>& movelist)
{
   unsigned int i;
   int score;
   vector<Move> line;
   vector<int> values(movelist.size());
   
   int tableValue = MINIMUM_SCORE;
   MoveRec tableMoveRec = gNullMoveRec;
   Move* tableMovePtr = 0;

   winningMove = false;
   
   // Prep for a search
   PrepareSearch(false);

   // Check for a good move in the transposition table
   mpTranTable[mSide]->ProbeTable(mSearchDepth, 
                                  MINIMUM_SCORE, MAXIMUM_SCORE,
                                  mpBoard->mHashKey, 
                                  tableValue, tableMoveRec);

   // Did we find a move?
   if (tableMoveRec.mPiecePtr != 0) {
      // Convert our move record to a new move object
      tableMovePtr = mpGame->CreateMove(tableMoveRec);
      PRINT_HASH_MOVE(tableMovePtr, tableMoveRec);
   }
   
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
      else if (tableMovePtr && (movelist[i] == *tableMovePtr)) {
         // If this move is indicated as a good move
         // in the transposition table then inflate
         // the score for this item
         movelist[i].mValue = MAXIMUM_SCORE-1;
         delete tableMovePtr;
         tableMovePtr = 0;
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

   // Reclaim the memory from the move constructed from
   // the transposition table
   if (tableMovePtr) {
      delete tableMovePtr;
   }

   if (!winningMove) {
      // Sort the moves by their scores
      sort( movelist.begin(), movelist.end(), ByValue<Move>() );

      // Reset the original move values
      for (i = 0; i < movelist.size(); i++) movelist[i].mValue = values[i];
   }
}

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  DoSearch
// Description:  Performs the actual alpha beta search
//--------------------------------------------------------------------------
int AlphaBetaMemory::DoSearch(int depth, int alpha, int beta, 
                              vector<Move>& pline)
{
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
      if (score != MAXIMUM_SCORE) {
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
            score = -DoSearch(depth+1, -beta, -alpha, line);
         }
         mpRepTable->RemoveKey( mpBoard->UnmakeMove(move) );
         PRINT_SCORE(depth, score, alpha, beta);
         if (mAbort) return 0;
         
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

//--------------------------------------------------------------------------
//       Class:  AlphaBetaMemory
//      Method:  DoQuiescenceSearch
// Description:  Performs the quiescence search
//--------------------------------------------------------------------------
int AlphaBetaMemory::DoQuiescenceSearch(int depth, int alpha, int beta,
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
