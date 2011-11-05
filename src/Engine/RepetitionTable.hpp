////////////////////////////////////////////////////////////////////////////
// Name:         RepetitionTable.hpp
// Description:  Interface for class that represents a small hash table
//               for detecting repetitions
// Created:      06/09/2004 08:35:42 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 02:34:20 $
// Revision:     $Revision: 1.1 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __RepetitionTable_HPP__
#define __RepetitionTable_HPP__

#include "common.hpp"

class RepetitionTable
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   RepetitionTable(int size);
   ~RepetitionTable();

   // ======================================================================
   // Operations
   // ======================================================================
   bool CheckForRepetition(uint64 key);
   void AddKey(uint64 key);
   void RemoveKey(uint64 key);
   void ClearTable();

private:
   // ======================================================================
   // Data
   // ======================================================================
   uint64* mpTable;
   int mSize;
};

#endif
