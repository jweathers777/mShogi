////////////////////////////////////////////////////////////////////////////
// Name:         Notator.cpp
// Description:  Implementation of class that provides notation for a game 
// Created:      05/09/2004 06:06:39 Eastern Daylight Time
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////

// Standard C++ header files
#include <iostream>
#include <sstream>
#include <iomanip>

// STL header files
#include <string>
#include <vector>

// mShogi header files
#include "common.hpp"
#include "Notator.hpp"
#include "Board.hpp"
#include "Move.hpp"
#include "Piece.hpp"
#include "PieceType.hpp"
#include "tokenize.hpp"

using std::string;
using std::istringstream;
using std::ostringstream;
using std::vector;

//--------------------------------------------------------------------------
//       Class:  Notator
//      Method:  Notator
// Description:  Construct an instance of a Notator object
//--------------------------------------------------------------------------
Notator::Notator(Board* board)
{
   mpBoard = board;
}

//--------------------------------------------------------------------------
//       Class:  Notator
//      Method:  ~Notator
// Description:  Destroys this instance of a Notator
//--------------------------------------------------------------------------
Notator::~Notator()
{
}


//-------------------------------------------------------------------------
//       Class:  Notator
//      Method:  ParseMove
// Description:  Parses a move string and returns a pointer to a new move
//-------------------------------------------------------------------------
Move* Notator::ParseMove(const string& movestring)
{
   int x, y;
   int width = mpBoard->GetWidth();
   int height = mpBoard->GetHeight();
   vector<int> squares;
   char cy, junk;
   bool finished = false;
   bool promote = false;

   istringstream instring(movestring);

   while (!finished) {
      instring >> x;
      if (!instring)
         return 0;
      
      x--;
      
      instring >> cy;
      if (!instring)
         return 0;
      
      y = (int)cy - (int)'a';

      if ( (x < 0) || (x > width) ||
           (y < 0) || (y > height) )
        return 0; 
         
      squares.push_back(x + width*y);

      junk = instring.peek();

      if (junk == '-') {
         instring >> junk;
      }
      else if (junk == 'x') {
         instring >> junk;
      }
      else if (junk == '+') {
         promote = true;
         finished = true;
         continue;
      }
      else {
         finished = true;
         continue;
      }
   }

   Piece* mover = mpBoard->mSquares[squares[0]];
   
   if (!mover)
      return 0;

   Move* movePtr;

   Piece* capture = 0;
   Piece* capture2 = 0;
   
   if (squares.size() == 2) {
      if (squares[0] != squares[1]) {
         capture = mpBoard->mSquares[squares[1]];
      }
      movePtr = new Move(mover, squares[0], squares[1], capture);
   }
   else if (squares.size() == 3) {
      vector<Piece*> captures;
      capture = mpBoard->mSquares[squares[1]];
      if (capture) {
         captures.push_back(capture);
      }
      if (squares[0] != squares[2]) {
         capture2 = mpBoard->mSquares[squares[2]];
         if (capture2) {
            captures.push_back(capture);
         }
      }
      if (capture) {
         movePtr = new Move(mover, squares[0], squares[1], squares[2], captures);
      }
      else {
         movePtr = new Move(mover, squares[0], squares[2], capture2);
      }
   }
   else 
      return 0;

   if (promote)
      movePtr->mPromote = true;

   return movePtr;
}


//-------------------------------------------------------------------------
//       Class:  Notator
//      Method:  Notate
// Description:  Takes a pointer to a move and returns notation for it
//-------------------------------------------------------------------------
string Notator::Notate(const Move* move)
{
   Piece* piecePtr = move->mpMover;
   PieceType* typePtr = piecePtr->mpType;

   string movestring("");
   
   if (move->mPromote) {
      movestring += piecePtr->mpRelatedType->mNotation;
   }
   else {
      movestring += piecePtr->mpType->mNotation;
   }

   movestring += SquareLabel(move->mSource);
      
   if (move->mCaptures.size() > 0) {
      movestring += "x";
   }
   else {
      movestring += "-";
   }

   movestring += SquareLabel(move->mDestination);

   if (move->mPromote) {
      movestring += "+";
   }

   return movestring;
}

//------------------------------------------------------------------------
//       Class:  Notator
//      Method:  Notate
// Description:  Takes a start square index and a finish square index
//               and returns a move string 
//------------------------------------------------------------------------
string Notator::Notate(int start, int finish)
{
   return SquareLabel(start) + '-' + SquareLabel(finish);
}

//-------------------------------------------------------------------------
//       Class:  Notator
//      Method:  Notate
// Description:  Takes a pointer to a board and returns notation for it
//-------------------------------------------------------------------------
string Notator::NotateBoard()
{
   using namespace std;
   
   int i, j, color;
   char colour[2] = { 'b', 'w' };
   Piece* piece;
   
   int width = mpBoard->GetWidth();
   int height = mpBoard->GetHeight();

   ostringstream output;
 
   // Print column labels
   output << endl << "    ";
   output << setiosflags(ios::left);
   for (i = width; i >= 1; i--) {
      output << " " << setw(2) << i;
      output << "   ";
   }
   output << endl;

   // Print board proper
   output << "  +";
   for (i = 1; i < width; i++) {
      output << "------";
   }
   output << "-----+" << endl;
   
   for (j = 0; j < height; j++) {
      char c = (char)(j + (int)'a');
      output << c << " |";
      for (i = width-1; i >= 0; i--) {
         piece = mpBoard->mSquares[i+width*j];
         if (piece) {
            color = piece->mColor;
            output << ' ' << colour[color] 
                   << setw(3) << setiosflags(ios::left) 
                   << piece->mpType->mNotation
                   << '|';
         }
         else {
            output << "     |";
         }
      }
      output << ' ' << c << endl;
      output << "  +";
      for (i = 1; i < width; i++) {
            output << "------";
         }
         output << "-----+" << endl;
   }

   // Print column labels
   output << "    ";
   for (i = width; i >= 1; i--) {
      output << " " << setw(2) << i;
      output << "   ";
   }
   output << endl << endl;
   output << "Key = " << mpBoard->mHashKey << endl;
   
   return output.str();
}


//-------------------------------------------------------------------------
//       Class:  Notator
//      Method:  SquareLabel
// Description:  Takes an integer square representation and returns a label
//-------------------------------------------------------------------------
string Notator::SquareLabel(int square)
{
   int x;
   char y;
   
   x = square % mpBoard->GetHeight() + 1;
   y = (char)((square / mpBoard->GetWidth()) + (int)'a');

   ostringstream buf;
   buf << x << y;

   return buf.str();
}
