////////////////////////////////////////////////////////////////////////////
// Name:         TranspositionTable.hpp
// Description:  Interface for class that represents a transposition table
// Created:      07/09/2004 06:10:54 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 02:34:20 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __TranspositionTable_HPP__
#define __TranspositionTable_HPP__

// STL header files
#include <string>

// Local include files
#include "common.hpp"
#include "MoveRec.hpp"

// Forward declarations
class Board;

class TranspositionTable
{
public:
   // ======================================================================
   // Constants
   // ======================================================================
   enum {
      NONE_FLAG=0x0,
      ALPHA_FLAG=0x1,
      BETA_FLAG=0x2,
      EXACT_FLAG=0x3
   };
   
   // ======================================================================
   // Structures
   // ======================================================================
   struct Entry
   {
      uint64 mKey;        // Board position signature
      int   mDepth;       // Relative depth of search that produced this entry
      int   mTimeStamp;   // "Time" stamp for this record's values
      char  mFlags;       // Indicates what kind of value is stored here
      int   mValue;       // Bound or value for this position's score
      MoveRec mBestMove;  // Best move found for this position
   };
   
   // ======================================================================
   // Class variables
   // ======================================================================
   static int sTimeStamp;
   static Entry sNullEntry;
      
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   TranspositionTable(int size);
   ~TranspositionTable();

   // ======================================================================
   // Operations
   // ======================================================================
   bool ProbeTable(int depth, int alpha, int beta, uint64 key,
                   int& value, MoveRec& bestmove);
   
   void StoreEntry(int depth, int value, uint64 key,
                   int flags, const MoveRec& bestmove);

   void ClearTable();

   void ClearStats();
   std::string GetStatistics();
   
private:
   bool IsCutOff(int depth, int alpha, int beta, 
                 Entry* entryPtr, int& value);
   
   // ======================================================================
   // Data
   // ======================================================================
   Entry* mpEntry;  // Pointer to an array of table entries
   int mSize;       // Number of entries in the table

   // Hit Statistics
   int mHits;
   int mCutOffHits;
   int mUpperBoundHits;
   int mLowerBoundHits;
   int mExactHits;
   int mMoveHits;
};

#endif

