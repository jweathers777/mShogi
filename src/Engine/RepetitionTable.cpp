////////////////////////////////////////////////////////////////////////////
// Name:         RepetitionTable.cpp
// Description:  Implementation for class that represents a small hash table
//               for detecting repetitions
// Created:      06/09/2004 10:05:31 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 02:34:20 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// Local header files
#include "RepetitionTable.hpp"

//--------------------------------------------------------------------------
//       Class:  RepetitionTable
//      Method:  RepetitionTable
// Description:  Construct an instance of a RepetitionTable object
//--------------------------------------------------------------------------
RepetitionTable::RepetitionTable(int size)
{
   mSize = size - 1;
   mpTable = new uint64 [size];
   for (int i = 0; i < size; i++) mpTable[i] = 0;
}

//--------------------------------------------------------------------------
//       Class:  RepetitionTable
//      Method:  ~RepetitionTable
// Description:  Destroys this instance of a RepetitionTable
//--------------------------------------------------------------------------
RepetitionTable::~RepetitionTable()
{
   if (mpTable) {
      delete [] mpTable;
      mpTable = 0;
   }
}


//-------------------------------------------------------------------------
//       Class:  RepetitionTable
//      Method:  CheckForRepetition
// Description:  Checks to see if the given hash key is already present
//               which indicates that a repetition has occurred
//-------------------------------------------------------------------------
bool RepetitionTable::CheckForRepetition(uint64 key)
{
   unsigned ikey;
   
   ikey = (unsigned)key & mSize;
   for (;;) {
      if (mpTable[ikey] == key)
         return true;
      if (!mpTable[ikey])
         return false;
      ikey = (ikey + 1) & mSize;
   }
}


//-------------------------------------------------------------------------
//       Class:  RepetitionTable
//      Method:  AddKey
// Description:  Adds a given hash key to the repetition table
//-------------------------------------------------------------------------
void RepetitionTable::AddKey(uint64 key)
{
   unsigned ikey;

   ikey = (unsigned)key & mSize;
   for (;;) {
      if (!mpTable[ikey]) {
         mpTable[ikey] = key;
         return;
      }
      ikey = (ikey + 1) & mSize;
   }
}


//-------------------------------------------------------------------------
//       Class:  RepetitionTable
//      Method:  RemoveKey
// Description:  Removes a hash key from the repetition table
//-------------------------------------------------------------------------
void RepetitionTable::RemoveKey(uint64 key)
{
   unsigned ikey;

   ikey = (unsigned)key & mSize;
   for (;;) {
      if (mpTable[ikey] == key) {
         mpTable[ikey] = 0;
         return;
      }
      ikey = (ikey + 1) & mSize;
   }
}

//-------------------------------------------------------------------------
//       Class:  RepetitionTable
//      Method:  ClearTable
// Description:  Removes the contents of the table
//-------------------------------------------------------------------------
void RepetitionTable::ClearTable()
{
   for (int i = 0; i < mSize; i++) mpTable[i] = 0;
}
