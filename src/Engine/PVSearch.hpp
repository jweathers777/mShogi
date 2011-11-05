////////////////////////////////////////////////////////////////////////////
// Name:         PVSearch.hpp
// Description:  Interface for class that performs a principle 
//               variation search
// Created:      07/09/2004 04:25:22 Eastern Standard Time
// Last Updated: $Date: 2004/08/28 03:22:18 $
// Revision:     $Revision: 1.2 $
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __PVSearch_HPP__
#define __PVSearch_HPP__

#include "AlphaBetaSearch.hpp"

class PVSearch : public AlphaBetaSearch
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   PVSearch() {};
   virtual ~PVSearch() {};

   // ======================================================================
   // Operations
   // ======================================================================
   Move* GetBestMove(int side);
   
private:
   int DoSearch(int depth, int alpha, int beta, std::vector<Move>& pline);
};

#endif

