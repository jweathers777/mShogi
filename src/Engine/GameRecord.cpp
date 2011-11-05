////////////////////////////////////////////////////////////////////////////////
// Name:         GameRecord.cpp
// Description:  Interface for a class that represents a game record
// Created:      Apr 20, 2004 4:03:50 PM
// Author:       John Weathers
// Email:        hotanguish@hotmail.com
// Copyright:    (c) 2004 John Weathers
////////////////////////////////////////////////////////////////////////////////

// Standard C++ headers
#include <iostream>
#include <fstream>
#include <sstream>

// STL headers
#include <string>
#include <vector>

// mShogi headers
#include "common.hpp"
#include "GameRecord.hpp"
#include "Game.hpp"
#include "Notator.hpp"
#include "Move.hpp"

using std::string;
using std::ofstream;
using std::ostringstream;
using std::endl;

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  GameRecord
// Description:  Constructs an instance of a game record
//--------------------------------------------------------------------------
GameRecord::GameRecord(Game* game)
{
   mpGame = game;
   mpNotator = game->mpNotator;
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  SetGame
// Description:  Set the game object and any dependent objects
//--------------------------------------------------------------------------
void GameRecord::SetGame(Game* game)
{
   mpGame = game;
   mpNotator = game->mpNotator;
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  AddMove
// Description:  Add a move to the record
//--------------------------------------------------------------------------
void GameRecord::AddMove(Move* move)
{
   mMoveList.push_back(*move);
   mMoveNotations.push_back(mpNotator->Notate(move));
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  PopMove
// Description:  Pop the last move from the record
//--------------------------------------------------------------------------
Move* GameRecord::PopMove()
{
   Move* move = new Move(mMoveList.back());
   mMoveList.pop_back();
   mMoveNotations.pop_back();
   
   return move;
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  LastMove
// Description:  Return a reference to the last move
//--------------------------------------------------------------------------
Move* GameRecord::LastMove()
{
   return &(mMoveList.back());
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  LastEntry
// Description:  Produce a string for the last move in the record
//--------------------------------------------------------------------------
string GameRecord::LastEntry()
{
   return mMoveNotations.back();
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  History
// Description:  Produce a string representing the game history
//--------------------------------------------------------------------------
string GameRecord::History()
{
   ostringstream output;
   
   for (unsigned int i = 0; i < mMoveNotations.size(); i++) {
      output << (i+1) << ". " << mMoveNotations[i] << endl;
	}
   
   return output.str();
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  Result
// Description:  Produce a string representing the game result
//--------------------------------------------------------------------------
string GameRecord::Result()
{
   ostringstream output;

   switch (mpGame->mStatus) {
   case Game::BLACK_WIN:
      output << "1-0";
      break;
   case Game::WHITE_WIN:
      output << "0-1";
      break;
   case Game::DRAW:
      output << "0.5-0.5";
   }
  
   return output.str();
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  Save
// Description:  Save the game record to a file
//--------------------------------------------------------------------------   
void GameRecord::Save(const string& filename, const string& gamename)
{
   ofstream output(filename.c_str());
   output << gamename << endl;
   output << History();
   
   switch (mpGame->mStatus) {
   case Game::BLACK_WIN:
      output << "1-0" << endl;
      break;
   case Game::WHITE_WIN:
      output << "0-1" << endl;
      break;
   case Game::DRAW:
      output << "0.5-0.5" << endl;
   }
   
   output.close();
}

//--------------------------------------------------------------------------
//       Class:  GameRecord
//      Method:  Reset
// Description:  Resets the game record to its initial state
//--------------------------------------------------------------------------
void GameRecord::Reset()
{
   mMoveList.clear();
   mMoveNotations.clear();
}


