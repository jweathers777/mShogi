////////////////////////////////////////////////////////////////////////////////
// Name:         Game.cpp
// Description:  Implementation for abstract class that represents a game
// Created:      Apr 20, 2004 10:18:12 AM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// Standard C++ header files
#include <string>

// mShogi header files
#include "common.hpp"
#include "Game.hpp"
#include "Board.hpp"
#include "MoveRec.hpp"
#include "MoveGenerator.hpp"
#include "Evaluator.hpp"
#include "Notator.hpp"

using std::vector;
using std::string;

//--------------------------------------------------------------------------
//       Class:  Game
//      Method:  ~Game
// Description:  Destroys an instance of a game
//--------------------------------------------------------------------------
Game::~Game()
{
   if (mpBoard) {
      delete mpBoard;
      mpBoard = 0;
   }
   
   if (mpMoveGenerator) {
      delete mpMoveGenerator;
      mpMoveGenerator = 0;
   }
   
   if (mpEvaluator) {
      delete mpEvaluator;
      mpEvaluator = 0;
   }
   
   if (mpNotator) {
      delete mpNotator;
      mpNotator = 0;
   }    
}

//------------------------------------------------------------------------
//       Class:  Game
//      Method:  IsPieceOnSquare
// Description:  Returns true if a piece is on the given square
//------------------------------------------------------------------------
bool Game::IsPieceOnSquare(int square)
{
   return mpBoard->mAllPieces.test(square);
}

//------------------------------------------------------------------------
//       Class:  Game
//      Method:  IsPieceColor
// Description:  Returns true if the piece on the give square 
//               is of the given color
//------------------------------------------------------------------------
bool Game::IsPieceColor(int square, int side)
{
   return mpBoard->mColoredPieces[side].test(square);
}

//------------------------------------------------------------------------
//       Class:  Game
//      Method:  GetPieceName
// Description:  Returns the unpromoted or promoted name associated with 
//               the given piece type in the given language
//------------------------------------------------------------------------
string Game::GetPieceName(int piecetype, int language, bool promoted)
{
   if (promoted) {
      piecetype = GetPromotedType(piecetype);
      if (piecetype == -1) {
         return "Does not promote";
      }
   }
      
   return mPieceTypes[piecetype]->mNames[language];
}

//--------------------------------------------------------------------------
//       Class:  Game
//      Method:  MakeMove
// Description:  Attempts to make the given move
//--------------------------------------------------------------------------
void Game::MakeMove(Move* move)
{
   mHashKeys.push_back(mpBoard->mHashKey);
   mpBoard->MakeMove(move);
   if ( GameWon(BLACK) ) {
      mStatus = BLACK_WIN;
   }
   else if ( GameWon(WHITE) ) {
      mStatus = WHITE_WIN;
   }
   else if ( GameDrawn() ) {
      mStatus = DRAW;
   }
   else {
      mStatus = IN_PROGRESS;
   }
}

//--------------------------------------------------------------------------
//       Class:  Game
//      Method:  UnmakeMove
// Description:  Attempts to unmake a move
//--------------------------------------------------------------------------
void Game::UnmakeMove(Move* move)
{
   mpBoard->UnmakeMove(move);
   mHashKeys.pop_back();
   mStatus = IN_PROGRESS;
}

//--------------------------------------------------------------------------
//       Class:  Game
//      Method:  ParseMove
// Description:  Parses a move string and constructs a new move object
//--------------------------------------------------------------------------  
Move* Game::ParseMove(const string& movestring)
{
   return mpNotator->ParseMove(movestring);
}

//--------------------------------------------------------------------------
//       Class:  Game
//      Method:  Notate
// Description:  Produces a string representation of a move
//--------------------------------------------------------------------------  
string Game::Notate(Move* move)
{
   return mpNotator->Notate(move);
}

//--------------------------------------------------------------------------
//       Class:  Game
//      Method:  PrintBoard
// Description:  Prints a standard text-based board representation
//--------------------------------------------------------------------------  
string Game::NotateBoard()
{
   return mpNotator->NotateBoard();
}

//------------------------------------------------------------------------
//       Class:  Game
//      Method:  GetPieceInfo
// Description:  Return a vector of PieceInfo structures for each square
//               on the board
//------------------------------------------------------------------------
void Game::GetPieceInfo(vector<PieceInfo>& squares)
{
   mpBoard->GetPieceInfo(squares);
}

//------------------------------------------------------------------------
//       Class:  Game
//      Method:  GetDestinationSquares
// Description:  Return a vector of integers representing possible
//               destination squares for the piece at the given 
//               board location
//------------------------------------------------------------------------
void Game::GetDestinationSquares(int index, vector<int>& squares)
{
   Piece* mover = mpBoard->mSquares[index];
   if (!mover) return;

   vector<Move> movelist;
   mover->mpType->GenerateCaptures(movelist, mover);
   mover->mpType->GenerateNonCaptures(movelist, mover);
   
   // Allow any game specific processing
   int color = mover->mColor;
   PostProcess(movelist, color);

   squares.resize(0);
   Move* movePtr;
   for (unsigned int i = 0; i < movelist.size(); i++) {
      movePtr = &(movelist[i]);
      if (!movePtr->mValidate || IsValid(movePtr, color)) {
         squares.push_back(movelist[i].mDestination);
         if (movelist[i].mVisited != -1) {
            squares.push_back(-1*movelist[i].mVisited);
         }
      }
   }
}

//------------------------------------------------------------------------
//       Class:  Game
//      Method:  GetSecondSquares
// Description:  Return a vector of integers representing possible
//               destination squares for the piece at the given 
//               board location when making a second part of a two
//               part move
//------------------------------------------------------------------------
void Game::GetSecondSquares(int start, int middle, vector<int>& squares)
{
   squares.clear();
}

