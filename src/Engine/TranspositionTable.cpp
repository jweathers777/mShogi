// Created:      07/14/2004 05:18:15 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 02:34:20 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// Standard C++ library
#include <cstring>
#include <iostream>
#include <sstream>

// Local header files
#include "TranspositionTable.hpp"

#ifdef DEBUG

#include <fstream>
#include <iomanip>

using std::setw;
using std::endl;

extern std::ofstream gLog;

char* strings[] = { "None", "Alpha", "Beta", "Exact" };

#define PRINT_HASH_CUTOFF(depth, type, value) \
if (keeplog) { \
   gLog << setw(2*depth) << " " << "Hash Table Cutoff: " \
        << type << ", " << value << endl; \
}

#define PRINT_OLD_HASH(depth, entryPtr) \
if (keeplog) { \
   gLog << setw(2*depth) << " " << "Old Hash Table Entry: " \
        << entryPtr->mKey << ", " \
        << entryPtr->mDepth << ", " \
        << entryPtr->mTimeStamp << ", " \
        << strings[entryPtr->mFlags] << ", " \
        << entryPtr->mValue << endl; \
}

#define PRINT_STORE_HASH(depth, entryPtr) \
if (keeplog) { \
   gLog << setw(2*depth) << " " << "Storing New Hash Entry: " \
        << entryPtr->mKey << ", " \
        << entryPtr->mDepth << ", " \
        << entryPtr->mTimeStamp << ", " \
        << strings[entryPtr->mFlags] << ", " \
        << entryPtr->mValue << endl; \
}

#define PRINT_DONT_STORE_HASH(depth, key, flags, value) \
if (keeplog) { \
   gLog << setw(2*depth) << " " << "Not Storing Hash Entry: " \
        << key << ", " \
        << depth << ", " \
        << sTimeStamp << ", " \
        << strings[flags] << ", " \
        << value << endl; \
}

#else

#define PRINT_HASH_CUTOFF(depth, type, value)
#define PRINT_OLD_HASH(depth, entryPtr)
#define PRINT_STORE_HASH(depth, entryPtr)
#define PRINT_DONT_STORE_HASH(depth, key, flags, value)
#endif

// =========================================================================
// Static member intialization
// =========================================================================

int TranspositionTable::sTimeStamp = 0;

TranspositionTable::Entry TranspositionTable::sNullEntry = {
       (uint64)0, // mKey
              -1, // mDepth
              -1, // mTimeStamp
               0, // mFlags
               0, // mValue
   {-1,-1,-1,0,0} // mBestMove
};

//--------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  TranspositionTable
// Description:  Construct an instance of a TranspositionTable object
//--------------------------------------------------------------------------
TranspositionTable::TranspositionTable(int size)
{
   mSize = size;
   mpEntry = new Entry [mSize];

   mHits = mCutOffHits = mUpperBoundHits = mLowerBoundHits = 0;
   mExactHits = mMoveHits = 0;

   // Fill the table with null entries
   for (int i = 0; i < mSize; i++) memcpy(&(mpEntry[i]), &sNullEntry, 1);
}

//--------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  ~TranspositionTable
// Description:  Destroys this instance of a TranspositionTable
//--------------------------------------------------------------------------
TranspositionTable::~TranspositionTable()
{
   if (mpEntry) {
      delete [] mpEntry;
      mpEntry = 0;
   }
}

//------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  ProbeTable
// Description:  Probe the table for an entry that matches the current
//               board position
//------------------------------------------------------------------------
bool TranspositionTable::ProbeTable(int depth, int alpha, int beta, 
                                    uint64 key, int& value, MoveRec& bestmove)
{
   bool result = false;
   
   Entry* entryPtr = &(mpEntry[key % mSize]);

   // Does our key match this entry's key?
   if (entryPtr->mKey == key) {
      mHits++;
      // Check for a cutoff
      if ( IsCutOff(depth, alpha, beta, entryPtr, value) ) {
         mCutOffHits++;
         result = true;
      }
      // Do we have a best move?
      else if (entryPtr->mBestMove.mPiecePtr != 0) {
         mMoveHits++;
         // Remember the best move so we can search it first
         memcpy(&bestmove, &(entryPtr->mBestMove), sizeof(MoveRec));
      }
   }
   
   return result;
}

//------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  StoreEntry
// Description:  Store an entry in the transposition table
//------------------------------------------------------------------------
void TranspositionTable::StoreEntry(int depth, int value, uint64 key,
                                    int flags, const MoveRec& bestmove)
{
   Entry* entryPtr = &(mpEntry[key % mSize]);

   // Only replace an entry if the new entry results from
   // a deeper or more recent search
   PRINT_OLD_HASH(depth, entryPtr);
   if ((sTimeStamp != entryPtr->mTimeStamp) ||
       (entryPtr->mDepth < depth)) {
      entryPtr->mKey = key;
      entryPtr->mDepth = depth;
      entryPtr->mTimeStamp = sTimeStamp;
      entryPtr->mFlags = flags;
      entryPtr->mValue = value;
      PRINT_STORE_HASH(depth, entryPtr);

      memcpy(&(entryPtr->mBestMove), &bestmove, sizeof(MoveRec));
   }
   else {
      PRINT_DONT_STORE_HASH(depth, key, flags, value);
   }
}


//-------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  ClearTable
// Description:  Clear the contents of the table
//-------------------------------------------------------------------------
void TranspositionTable::ClearTable()
{
   // Fill the table with null entries
   for (int i = 0; i < mSize; i++) memcpy(mpEntry+i, &sNullEntry, 1);

   // Reset the time stamp indicator
   sTimeStamp = 0;
   
   ClearStats();
}


//------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  ClearStats
// Description:  Re-initialize the statistics
//------------------------------------------------------------------------
void TranspositionTable::ClearStats()
{
   // Reset the hit statistics
   mHits = mCutOffHits = mUpperBoundHits = mLowerBoundHits = 0;
   mExactHits = mMoveHits = 0;
}

//------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  GetStatistics
// Description:  Return a string with a report about transposition table
//               statistics
//------------------------------------------------------------------------
std::string TranspositionTable::GetStatistics()
{
   using namespace std;

   ostringstream output;

   output << "Hits             = " << mHits << endl;
   output << "Cutoff Hits      = " << mCutOffHits << endl;
   output << "Lower Bound Hits = " << mLowerBoundHits << endl;
   output << "Exact Value Hits = " << mExactHits << endl;
   output << "Upper Bound Hits = " << mUpperBoundHits << endl;
   output << "Best Move Hits   = " << mMoveHits << endl;
   
   ClearStats();

   return string(output.str());
}
//------------------------------------------------------------------------
//       Class:  TranspositionTable
//      Method:  IsCutOff
// Description:  Check whether we have a cutoff 
//------------------------------------------------------------------------
bool TranspositionTable::IsCutOff(int depth, int alpha, int beta,
                                  TranspositionTable::Entry* entryPtr, 
                                  int& value)
{
   bool result = false;
   
   // Make sure we've improved on the depth of our last entry
   // and that we're not too close to the root
   if ((depth > 2) && (depth <= entryPtr->mDepth)) {
      if (entryPtr->mFlags & ALPHA_FLAG) {
         // Is this an exact value?
         if (entryPtr->mFlags & BETA_FLAG) {
            value = entryPtr->mValue;
            mExactHits++;
            result = true;
            PRINT_HASH_CUTOFF(depth, "Exact Score", value);
         }
         // Is this an alpha value that's at most our current alpha?
         else if (entryPtr->mValue <= alpha) {
            // Return an upper bound on the score
            value = alpha;
            mUpperBoundHits++;
            result = true;
            PRINT_HASH_CUTOFF(depth, "Upper Bound", value);
         }
      }
      else if (entryPtr->mFlags & BETA_FLAG) {
         // Is this a beta value that's at least our current beta
         if (entryPtr->mValue >= beta) {
            // Return a lower bound on the score
            value = beta;
            mLowerBoundHits++;
            result = true;
            PRINT_HASH_CUTOFF(depth, "Lower Bound", value);
         }
      }
   }

   return result;
}
