////////////////////////////////////////////////////////////////////////////
// Name:         IterDeepSearch.hpp
// Description:  Template class that performs an interative deepening
//               with an alpha beta search
// Created:      07/17/2004 04:09:22 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 03:01:47 $
// Revision:     $Revision: 1.2 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __IterDeepSearch_HPP__
#define __IterDeepSearch_HPP__

// Standard C++ include files
#include <iostream>

// Local include files
#include "Search.hpp"
#include "Move.hpp"

// BaseSearch must be a subclass of Search
template <class BaseSearch>
class IterDeepSearch : public BaseSearch
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   IterDeepSearch() {};
   virtual ~IterDeepSearch() {};

   // ======================================================================
   // Operations
   // ======================================================================
   Move* GetBestMove(int side);
};

//-------------------------------------------------------------------------
//       Class:  IterDeepSearch
//      Method:  GetBestMove
// Description:  Return the best move available for the given side as
//               determined by the search algorithm
//-------------------------------------------------------------------------
template <class BaseSearch>
Move* IterDeepSearch<BaseSearch>::GetBestMove(int side)
{
   Move* bestmove = 0;
   Move* moveptr;
   int maxdepth = BaseSearch::mSearchDepth;
   
   double seconds = 0.0;
   int nodes = 0;
   int cutoffs = 0;
   int qnodes = 0;
   int qcutoffs = 0;

   BaseSearch::mSearchDepth = 1;
   do {
      std::cout << "Search depth = " << BaseSearch::mSearchDepth << std::endl;
      moveptr = BaseSearch::GetBestMove(side);

      // Stop searching if we've aborted
      if (BaseSearch::mAbort) break;
 
      if (bestmove) {
         delete bestmove;
      }
      bestmove = moveptr;
     
      seconds  += BaseSearch::mSeconds;
      cutoffs  += BaseSearch::mCutOffs;
      qnodes   += BaseSearch::mQuiescentNodes;
      qcutoffs += BaseSearch::mCutOffs;
      
      BaseSearch::mSearchDepth++;
   } while (BaseSearch::mSearchDepth <= maxdepth);

   // Restore original value to mSearchDepth
   BaseSearch::mSearchDepth = maxdepth;

   BaseSearch::mSeconds = seconds;
   BaseSearch::mCutOffs = cutoffs;
   BaseSearch::mQuiescentNodes = qnodes;
   BaseSearch::mQuiescentCutOffs = qcutoffs;

   return bestmove;
}
#endif
