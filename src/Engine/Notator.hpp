////////////////////////////////////////////////////////////////////////////
// Name:         Notator.hpp
// Description:  Interface for class that provides notation for a game 
// Created:      05/09/2004 06:03:44 Eastern Daylight Time
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

#ifndef __Notator_HPP__
#define __Notator_HPP__

// STL Header files
#include <string>

class Board;
class Move;

class Notator
{
public:
   // ======================================================================
   // Construction and destruction
   // ======================================================================
   Notator(Board* board);
   virtual ~Notator();

   // ======================================================================
   // Operations
   // ======================================================================
   Move* ParseMove(const std::string& movestring);
   
   std::string Notate(const Move* move);
   std::string Notate(int start, int finish);
   std::string NotateBoard();
   
private:
   std::string SquareLabel(int square);
   
   // ======================================================================
   // Data
   // ======================================================================
   Board* mpBoard;
};

#endif
