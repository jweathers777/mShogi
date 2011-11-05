////////////////////////////////////////////////////////////////////////////
// Name:         PVMSearch.hpp
// Description:  Interface for class that performs a memory enhanced
//               principle variation search
// Created:      07/28/2004 09:58:51 Eastern Daylight Time
// Last Updated: $Date: 2004/08/28 03:22:18 $
// Revision:     $Revision: 1.2 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __PVMSearch_HPP__
#define __PVMSearch_HPP__

#include "AlphaBetaMemory.hpp"

class PVMSearch : public AlphaBetaMemory
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   PVMSearch() {};
   virtual ~PVMSearch() {};

   // ======================================================================
   // Operations
   // ======================================================================
   Move* GetBestMove(int side);
   
private:
   int DoSearch(int depth, int alpha, int beta, std::vector<Move>& pline);
};

#endif
