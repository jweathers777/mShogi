////////////////////////////////////////////////////////////////////////////////
// Name:         Search
// Description:  Interface for class that represents a game tree search
// Created:      Apr 20, 2004 10:56:06 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////
#ifndef SEARCH_HPP
#define SEARCH_HPP

#include <string>

#ifdef __WXMSW__   
// MSWindows
#include <windows.h>

typedef DWORD PRECISION_TIME;

#define GET_PRECISION_TIME(t) \
   t = GetTickCount()

#define DIFF_PRECISION_TIME(diff, t1, t0) \
   diff = (double)(t1 - t0) / 1000

#else       
// Linux or Mac OS X
#include <sys/time.h>

typedef struct timeval PRECISION_TIME;

#define GET_PRECISION_TIME(t) gettimeofday(&t, NULL)

#define DIFF_PRECISION_TIME(diff, t1, t0) \
   int seconds_diff = t1.tv_sec - t0.tv_sec; \
   int microseconds_diff; \
   if (t1.tv_usec < t0.tv_usec) { \
      microseconds_diff = 1000000 + t1.tv_usec - t0.tv_usec; \
      --seconds_diff; \
   } \
   else { \
      microseconds_diff = t1.tv_usec - t0.tv_usec; \
   } \
   diff = (double)((microseconds_diff / 10000) + (seconds_diff * 1000)) / 1000

#endif

// Forward declarations
class Engine;
class Game;
class Move;

class Search
{
public:
   // ======================================================================
   // Accessors
   // ======================================================================
   virtual void SetEngine(Engine* engine) = 0;
   virtual void SetGame(Game* game) = 0;
   
   virtual void SetHashSize(int size) = 0;
   
   void SetSearchTime(int time) { mSearchTime = time; }
   void SetSearchDepth(int depth) { mSearchDepth = depth; }
   void SetSearchNodes(int nodes) { mSearchNodes = nodes; }
   
   void SetQuiescence() { mQuiescenceOn = !mQuiescenceOn; }
   void SetQuiescence(bool value) { mQuiescenceOn = value; }
   void SetQuiescenceDepth(int depth) { mQuiescenceDepth = depth; }
   
   int GetSearchTime() { return mSearchTime; }
   
   // ======================================================================
   // Operations
   // ======================================================================
   virtual Move* GetBestMove(int side) = 0;
   virtual std::string GetStatistics() = 0;
   virtual void Reset() = 0;
   
protected:
   // ======================================================================
   // Constants
   // ======================================================================
   enum { 
      DEFAULT_SEARCH_TIME = 300,
      DEFAULT_SEARCH_DEPTH = 6,
      DEFAULT_SEARCH_NODES = 1000000,
      NODES_BETWEEN_TIME_CHECKS = 300,
      DRAW_SCORE = 0,
      REPETITION_SCORE = 5000000,
      MAXIMUM_SCORE = 10000000,
      MINIMUM_SCORE = -10000000
   };

   // ======================================================================
   // Data
   // ======================================================================
   int  mSearchTime;       // Maximum time allowed for search in seconds
   int  mSearchDepth;      // Maximum depth of search tree
   int  mSearchNodes;      // Maximum nodes searched in tree
   bool mQuiescenceOn;     // Set to true for quiescence search
   int  mQuiescenceDepth;  // Quiescence search depth
};


#endif // SEARCH_HPP
